# extra-utils

This repository contains some handy command line utilities that aren't included in the
GNU coreutils. They follow the UNIX philosophy.

A lot of the tools here already exist in other languages/packages. The purpose of this
project is more for fun/learning than it is for actual innovation or utility.

## Installation
```
cargo install extra-utils
```

## Commands

### ts
Append a timestamp to each line in STDIN

Usage: `your_command | ts [format]`

The default format is "[%Y-%m-%d %H:%M:%S]"

### nth
Select the nth line(s) from STDIN

Usage: `your_command | nth <start> [end]`

Where `start` is the starting line to print, and `end` is the optional end index to
print. If `end` is omitted, only the `start`'th line will be printed
