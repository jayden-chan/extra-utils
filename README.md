# extra-utils [![crates.io][Crate Logo]][Crate]

This repository contains some handy command line utilities that aren't included in the
GNU coreutils. They follow the UNIX philosophy.

A lot of the tools here already exist in other languages/packages. The purpose of this
project is more for fun/learning than it is for actual innovation or utility.

## Installation
```
cargo install extra-utils
```

## Commands

### se
A simple HTTP server that serves a static folder

Usage: `se [port] [folder]`

The default port is 3000 and the default folder is `.` (current folder)

If only one argument is provided, its purpose will be inferred:
```
se build
```
will serve the "build" folder on port 3000 and:
```
se 8080
```
will serve the current folder on port 8080.

If the requested file doesn't exist, `se` will attempt to fallback to
`{folder}/404.html`. Failing that, it will return HTTP `404` with an empty body.

### ts
Prepend a timestamp to each line in STDIN

Usage: `your_command | ts [format]`

The default format is "[%Y-%m-%d %H:%M:%S]"

### nth
Select the nth line(s) from STDIN

Usage: `your_command | nth <start> [end]`

Where `start` is the starting line to print, and `end` is the optional end index to
print. If `end` is omitted or is less than `start`, only the `start`'th line will be
printed

### nr
Prepend line line numbers to STDIN

Usage: `your_command | nr [minwidth] [offset]`

`minwidth` is the minimum width of the number column. Default: 0

`offset` is the starting index of the line numbers. Default: 1

[Crate]: https://crates.io/crates/extra-utils
[Crate Logo]: https://img.shields.io/crates/v/extra-utils?style=flat-square
