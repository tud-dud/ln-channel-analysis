#!/bin/bash

# block height closest to 6/2/2026:0000
MIN_HEIGHT=935175
OUTPUT_FILE="chanpoints_filtered.csv"

echo "funding_txid,output_index,block_height" > "$OUTPUT_FILE"

lncli describegraph \
| jq -r --argjson min "$MIN_HEIGHT" '
    .edges
    | unique_by(.chan_point)
    | .[]
    | (.channel_id | tonumber) as $cid
    | ($cid / 1099511627776 | floor) as $height
    | select($height >= $min)
    | (.chan_point | split(":")) as $cp
    | [$cp[0], $cp[1], $height]
    | @csv
' >> "$OUTPUT_FILE"

echo "Saved filtered chan_points to $OUTPUT_FILE"
