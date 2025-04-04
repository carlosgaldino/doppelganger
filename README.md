# doppelganger

This script is used to find occasions where the same set of CPUs were used at
the same time by multiple cells.

Entries are grouped by instance, cpu set, and 1 second bucket.

## Usage

``` shell
$ cargo run path/to/data.csv
```

The CSV must have the following schema:

``` csv
Date,@cpuset,@instance_id,cell_id
```

The first line must be the header.
