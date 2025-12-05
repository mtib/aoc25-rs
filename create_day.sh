#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

# Get the day number as integer and zero-padded
day_num=$1
day_padded=$(printf "%02d" $day_num)

# Template file path
template_file="src/day/.example_day.rs"

# Output file path
output_file="src/day/day_${day_padded}.rs"

# Check if template exists
if [ ! -f "$template_file" ]; then
    echo "Error: Template file $template_file not found"
    exit 1
fi

# Check if output file already exists
if [ -f "$output_file" ]; then
    echo "Error: File $output_file already exists"
    exit 1
fi

# Replace DayXX with DayNN (zero-padded) and standalone XX with N (not padded)
sed -e "s/DayXX/Day${day_padded}/g" -e "s/XX/${day_num}/g" "$template_file" > "$output_file"

echo "Created $output_file"
