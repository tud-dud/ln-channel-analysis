# channel-frequency

1. Analyse the frequency of the channel openings provided in the a CSV file:

```bash
    target/release/channel-frequency [OPTIONS] --input <CHANPOINTS>
    
    Options:
      -i, --input <CHANPOINTS>  Path to the CSV with the channel points
      -l, --log <LOG_LEVEL>     [default: info]
      -o, --out <OUTPUT_PATH>   [default: ./results]
      -h, --help                Print help
```
