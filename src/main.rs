use clap::Parser;
use core::panic;
use std::io::ErrorKind;
use std::path::PathBuf;

/// A simple program to convert notes from either Apple Notes or Simplenote to markdown which can be used with Notable or other editors.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    /// a JSON file path will be parsed as Simplenote data; a directory path will be parsed as Apple Notes data
    #[clap(short, long)]
    source_path: PathBuf,

    /// the writeable directory path where converted note files will be written
    #[clap(short, long)]
    dest_dir: PathBuf,
}

fn main() {
    let args = Args::parse();

    // std::process::exit(match notes2md::process_source(args.source_path) {
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => 1,
    //         ErrorKind::PermissionDenied=> 2,
    //         _ => panic!("Unhandled error {:?}", e)
    //     },
    //     Ok(_) => 0,
    // })
}