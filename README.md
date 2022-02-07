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

Currently this functions quite well for Simplenote conversions.
A note will be ignored if it has no content, more spcecifically, if the title parsing results in an empty string. If this occurs, the resulting converted markdown is output with an error message.


```bash
$ ./notes2md -d test_data/out simplenote ../notes.json
notes2md will read simplenote from source '../notes.json' and write to 'test_data/out'
ERROR processing Note:
---
title: ""
created: "2021-02-15T17:04:31.319Z"
modified: "2021-02-15T17:05:25.325Z"
---


title: '' is not valid for a filename
```

Apple Notes is not yet implemented.

```bash
$ ./notes2md -d /tmp applenotes ./test_data/
notes2md will read applenotes from source './test_data/' and write to '/tmp'
Apple Notes conversion not yet implemented.
```
