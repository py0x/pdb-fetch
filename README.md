# pdb-fetch

```
$ pdb-fetch -h
                                                                                                                                                [bd9d7a9]
pdb-fetch 0.1.0
PY <py@pypy.info>
Download Multiple Files from the PDB Archive.

USAGE:
pdb-fetch [OPTIONS] --file <FILE>

OPTIONS:
-f, --file <FILE>                  The input file containing a newline-separated list of PDB ids
-o, --output <OUTPUT>              The output dir, default: current dir
--format <FORMAT>              Download file format each PDB id [default: pdb] [possible
values: pdb, cif]
--concurrency <CONCURRENCY>    Number of concurrency for batch downloading [default: 80]
-e, --err-file <ERR_FILE>          The file for error PDB ids, default: "error.pdb-id.log"
-h, --help                         Print help information
-V, --version                      Print version information
```