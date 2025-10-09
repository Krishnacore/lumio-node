#!/bin/bash

echo "Fixing YAML and ignore files..."

# Function to replace aptos with lumio in file, preserving copyright
fix_file() {
    local file="$1"
    echo "Processing: $file"
    
    # Create temporary file
    tmp_file="${file}.tmp"
    
    # Process the file
    awk '
    /Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier/ { 
        print $0 
    }
    !/Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier/ { 
        # Don't replace in URLs to aptos-labs repos
        if ($0 ~ /github\.com\/aptos-labs/) {
            print $0
        } else {
            gsub(/Aptos/, "Lumio")
            gsub(/APTOS/, "LUMIO")  
            gsub(/aptos/, "lumio")
            print $0
        }
    }' "$file" > "$tmp_file"
    
    # Replace file if changed
    if ! cmp -s "$file" "$tmp_file"; then
        mv "$tmp_file" "$file"
        echo "  Updated: $file"
    else
        rm "$tmp_file"
    fi
}

# Process .gitignore and .dockerignore
for file in $(find . -type f \( -name ".gitignore" -o -name ".dockerignore" \) ! -path "*/target/*" ! -path "*/.git/*"); do
    fix_file "$file"
done

# Process YAML files
for file in $(find . -type f \( -name "*.yaml" -o -name "*.yml" \) ! -path "*/target/*" ! -path "*/.git/*"); do
    fix_file "$file"
done

echo "Completed fixing YAML and ignore files"
