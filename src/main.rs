use std::fs::{self};
use std::os::unix::fs::MetadataExt;
use std::io;
use std::path::{Path, PathBuf};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use walkdir::WalkDir;
use users::{get_user_by_uid, get_group_by_gid};
use clap::{Arg, Command};

/// Trait for displaying information with color.
trait DisplayWithColor {
    /// Displays the information with color and indentation based on depth.
    ///
    /// # Arguments
    ///
    /// * `depth` - The depth of the file or directory in the hierarchy.
    fn display_with_color(&self, depth: usize);
}

/// Struct to hold file information.
struct FileInfo {
    name: String,
    file_type: String,
    owner: String,
    group: String,
    permissions: String,
    color: Color,
}

impl DisplayWithColor for FileInfo {
    fn display_with_color(&self, depth: usize) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(Some(self.color));
        stdout.set_color(&color_spec).unwrap();

        // Print with indentation and symbols based on depth
        let indent = "  ".repeat(depth);
        let name_display = if depth > 0 {
            format!("{}> {}", indent, self.name)
        } else {
            self.name.clone()
        };

        println!(
            "{:<60} {:<10} {:<20} {:<20} {:<10}",
            name_display,
            self.file_type,
            self.owner,
            self.group,
            self.permissions
        );

        stdout.reset().unwrap();
    }
}

/// Lists files and directories with color-coded output.
///
/// # Arguments
///
/// * `paths` - A slice of PathBuf representing the paths to list.
/// * `recursive` - A boolean flag indicating whether to list directories recursively.
/// * `show_hidden` - A boolean flag indicating whether to show hidden files.
///
/// # Returns
///
/// * `io::Result<()>` - Result indicating success or failure.
fn list_files_and_dirs(paths: &[PathBuf], recursive: bool, show_hidden: bool) -> io::Result<()> {
    let mut has_entries = false; // Track if any files or directories are found

    // Print header
    println!("{:<60} {:<10} {:<20} {:<20} {:<10}", "Name", "Type", "Owner", "Group", "Permissions");

    for path in paths {
        let path_str = path.to_str().unwrap_or("Unknown");
        println!("\nListing in: {}", path_str);

        // Process each directory
        for entry in WalkDir::new(path)
            .max_depth(if recursive { usize::MAX } else { 1 }) // Adjust depth based on recursive flag
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            let depth = entry.depth(); // Current depth in recursion

            if !show_hidden && path.file_name().unwrap_or_default().to_str().map_or(false, |s| s.starts_with('.')) {
                continue;
            }
            if path.is_file() {
                has_entries = true; // Found at least one file
                let file_info = create_file_info(path, Color::Green);
                file_info.display_with_color(depth);
            } else if path.is_dir() {
                has_entries = true; // Found at least one directory
                let file_info = create_file_info(path, Color::Blue);
                file_info.display_with_color(depth);
            }
        }
    }

    if !has_entries {
        println!("No files or directories found.");
    }

    Ok(())
}

/// Creates a FileInfo struct for a given path.
///
/// # Arguments
///
/// * `path` - A reference to a Path representing the file or directory.
/// * `color` - The color to use for displaying the file or directory.
///
/// # Returns
///
/// * `FileInfo` - Struct containing file information.
fn create_file_info(path: &Path, color: Color) -> FileInfo {
    let metadata = fs::metadata(path).unwrap();
    let file_type = if metadata.is_dir() { "Directory" } else { "File" }.to_string();
    let owner = get_user_by_uid(metadata.uid()).map_or("Unknown".to_string(), |u| u.name().to_string_lossy().into_owned());
    let group = get_group_by_gid(metadata.gid()).map_or("Unknown".to_string(), |g| g.name().to_string_lossy().into_owned());
    let permissions = format!("{:o}", metadata.mode() & 0o777);

    FileInfo {
        name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
        file_type,
        owner,
        group,
        permissions,
        color,
    }
}

fn main() {
    let mut cmd = Command::new("file_lister")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Lists files and directories with color-coded output")
        .override_usage("Usage: bls <PATHS> <optional>")
        .arg(Arg::new("recursive")
            .short('r')
            .long("recursive")
            .action(clap::ArgAction::SetTrue)
            .help("List directories recursively"))
        .arg(Arg::new("hidden")
            .short('x')
            .long("hidden")
            .action(clap::ArgAction::SetTrue)
            .help("Show hidden files"))
        .arg(Arg::new("paths")
            .value_name("PATHS")
            .help("Paths to list")
            .required(false)
            .num_args(1..));

    let matches = cmd.clone().get_matches();

    if matches.get_many::<String>("paths").is_none() {
        cmd.print_help().unwrap();
        return;
    }

    let paths: Vec<PathBuf> = matches.get_many::<String>("paths").unwrap().map(PathBuf::from).collect();
    let recursive = matches.get_flag("recursive");
    let show_hidden = matches.get_flag("hidden");

    if let Err(e) = list_files_and_dirs(&paths, recursive, show_hidden) {
        eprintln!("Error listing files and directories: {}", e);
    }
}
