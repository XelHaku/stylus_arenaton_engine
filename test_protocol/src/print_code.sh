#!/bin/bash

# Define the output file
output_file="rs_files_content.txt"

# Clear the output file if it already exists
> "$output_file"

# Find all .rs files and append their location and content to the output file
find . -name "*.rs" | while read -r file; do
    echo "File: $file" >> "$output_file"
    echo "--------------------" >> "$output_file"
    cat "$file" >> "$output_file"
    echo >> "$output_file"
    echo >> "$output_file"
done

echo "All .rs files' content has been written to $output_file."
