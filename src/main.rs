use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_help_flag=true)]
struct Args {
    /// The directory to list files and directories in
    #[arg(default_value = ".")]
    directory: String,

    /// Show hidden files and directories
    #[arg(short, long)]
    all: bool,

    /// Display file sizes in human-readable format
    #[arg(short = 'h', long)]
    human_readable: bool,

    /// Display file sizes in bytes
    #[arg(short = 's', long)]
    size: bool,

    /// Display directories as plain files, without trailing slashes
    #[arg(long)]
    no_trailing_slash: bool,

    /// Only list directories
    #[arg(short, long)]
    directory_only: bool,

    /// Only list files
    #[arg(short, long)]
    file_only: bool,
}

fn main() {
    let args = Args::parse();

    let dir_entries = fs::read_dir(&args.directory).unwrap();

    for entry in dir_entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();

        // Skip hidden files and directories if `--all` is not set
        if !args.all && file_name.starts_with(".") {
            continue;
        }

        let file_type = entry.file_type().unwrap();
        let file_type_str = if file_type.is_dir() {
            if args.no_trailing_slash {
                ""
            } else {
                "/"
            }
        } else {
            ""
        };

        if args.directory_only && !file_type.is_dir() {
            continue;
        }

        if args.file_only && file_type.is_dir() {
            continue;
        }

        let file_size = entry.metadata().unwrap().len();
        let file_size_str = if args.size {
            format!("{}", file_size)
        } else if args.human_readable {
            humantime::format_duration(std::time::Duration::from_secs(file_size))
                .to_string()
        } else {
            "".to_string()
        };

        println!(
            "{}{}{}",
            file_type_str,
            if args.human_readable || args.size {
                format!("{:>10} ", file_size_str)
            } else {
                "".to_string()
            },
            file_name
        );
    }
}
