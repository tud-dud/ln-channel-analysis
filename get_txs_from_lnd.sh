#!/bin/bash

# block height closest to 6/2/2026:0000
MIN_HEIGHT=935175
OUTPUT_FILE="chanpoints.csv"

# Write header only if file doesn't exist
if [ ! -f "$OUTPUT_FILE" ]; then
    echo "funding_txid,output_index" > "$OUTPUT_FILE"
fi

lncli describegraph \
| jq -r --argjson min "$MIN_HEIGHT" '
    .edges
    | unique_by(.chan_point)
    | .[]
    | (.channel_id | tonumber) as $cid
    | ($cid / 1099511627776 | floor) as $height
    | select($height >= $min)
    | (.chan_point | split(":"))
    | join(",")
' >> "$OUTPUT_FILE"

echo "Saved filtered chan_points to $OUTPUT_FILE"

# remove duplicate channels
awk -i inplace '!seen[$0]++' $OUTPUT_FILE
