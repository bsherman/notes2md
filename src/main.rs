use clap::{AppSettings, Parser, Subcommand};
use std::io::ErrorKind;
use std::path::PathBuf;

/// A simple program to convert notes from either Apple Notes or Simplenote to markdown which can be used with Notable or other editors.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    source_type: SourceTypes,

    /// directory where converted notes will be written
    #[clap(short, long)]
    dest_dir: String,
}

#[derive(Subcommand, Debug)]
enum SourceTypes {
    /// process an iCloud export directory of Apple Notes data <SOURCE_DIR>
    Applenotes { source_dir: String },
    /// process a JSON file export of Simplenote data <SOURCE_FILE>
    Simplenote { source_file: String },
}

fn main() {
    let cli = Cli::parse();

    let results = match &cli.source_type {
        SourceTypes::Applenotes { source_dir } => {
            println!(
                "notes2md will read applenotes from source '{}' and write to '{}'",
                source_dir, &cli.dest_dir
            );
            notes2md::process_applenotes(PathBuf::from(source_dir), PathBuf::from(cli.dest_dir))
        }
        SourceTypes::Simplenote { source_file } => {
            println!(
                "notes2md will read simplenote from source '{}' and write to '{}'",
                source_file, &cli.dest_dir
            );
            notes2md::process_simplenote(PathBuf::from(source_file), PathBuf::from(cli.dest_dir))
        }
    };

    std::process::exit(match results {
        Err(e) => match e.kind() {
            ErrorKind::InvalidData => {
                println!("{}", e);
                1
            }
            ErrorKind::InvalidInput => {
                println!("{}", e);
                2
            }
            ErrorKind::NotFound => {
                println!("{}", e);
                3
            }
            ErrorKind::PermissionDenied => {
                println!("{}", e);
                4
            }
            _ => panic!("Unhandled error {:?}", e),
        },
        Ok(_) => 0,
    })
}
