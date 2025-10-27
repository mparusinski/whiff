NAME
====

Whiff - A rust reimplementation of touch

USAGE
=====

`whiff [OPTIONS] [FILE]...`

DESCRIPTION
===========

Whiff is a rust reimplementation of touch
*TODO: Improve this documentation

OPTIONS
=======

`-h --help`

: Show the help message and exit

`-a`

: Change only the access time

`-c, --no-create`

: Do not create any files

`-d, --date DATE`

: Parase the DATE and ust it instead of current time

`-f`

: (ignored)

`-n --no-dereference`

: affect eacjh symbolic link instead of any reference file (useful only on systems that can change the timestamps of a symlink

`-m`

: Change only the modification time

`-r, --reference FILEPATH`

: Use another file's time as reference instead of current time

`-t SPECIFIED_TIME`

: Use specified time (format [[CC]YYMMDDhhmm[.ss]) instead of current time, with a date-time format that differs from -d's

`--time WORD`

: Specify which time to change: access (-a): 'access', 'atime', 'use'; modification time (-m): 'modify', 'mtime'

`-V, --version`

: Print version of the software

MORE INFORMATION
================

More information can be found here https://github.com/mparusinski/whiff
