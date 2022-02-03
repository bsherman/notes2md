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

Currently this will parse arguments but not actually process the any data.

```bash
$ ./notes2md -d /tmp applenotes ./test_data/
notes2md will read applenotes from source './test_data/' and write to '/tmp'
Apple Notes conversion not yet implemented.

$ ./notes2md -d test_data/out simplenote ../notes.json
notes2md will read simplenote from source '../notes.json' and write to 'test_data/out'
Simplenote conversion not yet implemented.
Parsing the data works... writing it has not begun.
Simplenote active_notes:597, trashed_notes:17.
---
title: A title
created: "2022-01-13T22:36:18.906Z"
modified: "2022-01-14T07:36:50.656Z"
---
This is a
great piece of
sample content!
```
