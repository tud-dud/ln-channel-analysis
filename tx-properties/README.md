# tx-properties

1. Analyse transactions provided in a CSV file and store results to a JSON file:

```bash
    target/release/channel-properties --input <CHANPOINTS>

    Options:
      -i, --input <CHANPOINTS>  Path to the CSV with the channel points
      -l, --log <LOG_LEVEL>     [default: info]
      -o, --out <OUTPUT_PATH>   Path to directory in which the results will be
                                stored [default: ./results]
      -h, --help                Print help
```

2. Analyse transactions in a block range excluding transactions provided in a
   CSV file and store results to a JSON file:

```bash
    target/release/btc-properties --input <LN_CHANPOINTS> --start <START> --end <END>

    Options:
      -i, --input <LN_CHANPOINTS>  Path to the CSV with the channel points
      -l, --log <LOG_LEVEL>        [default: info]
      -o, --out <OUTPUT_PATH>      Path to directory in which the results will
                                    be stored [default: ./results]
      -s, --start <START>          Block height to start at
      -e, --end <END>              Block height to end at (inclusive)
      -r, --rng <SEED>             Seed for the rng [default: 4711]
      -n, --num <NUM_BLOCKS>       Number of blocks to analyse. Pass '0' to
                                    analyse all [default: 100]
      -h, --help                   Print help
```
