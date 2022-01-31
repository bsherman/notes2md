# notes2md

## WORK IN PROGRESS

A simple utility to convert Apple Notes (exported from iCloud) or Simplenotes (exported to JSON) into markdown that can be used by something like [Notable](https://notable.app/).

This is mostly a toy project on which to practice [Rust](https://www.rust-lang.org/).

```bash
$ ./notes2md -h
notes2md 0.1.0
A simple program to convert notes from either Apple Notes or Simplenote to markdown which can be
used with Notable or other editors.

USAGE:
    notes2md --dest-dir <DEST_DIR> <SUBCOMMAND>

OPTIONS:
    -d, --dest-dir <DEST_DIR>    directory where converted notes will be written
    -h, --help                   Print help information
    -V, --version                Print version information

SUBCOMMANDS:
    applenotes    process an iCloud export directory of Apple Notes data <SOURCE_DIR>
    help          Print this message or the help of the given subcommand(s)
    simplenote    process a JSON file export of Simplenote data <SOURCE_FILE>
```