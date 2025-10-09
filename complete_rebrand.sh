#!/bin/bash

# Function to replace in file, preserving copyright/license headers
replace_in_file() {
    local file="$1"
    
    # Skip if file doesn't exist or is in excluded directories
    if [[ ! -f "$file" ]] || [[ "$file" == *"/target/"* ]] || [[ "$file" == *"/.git/"* ]]; then
        return
    fi
    
    # Create a temporary file
    tmp_file="${file}.tmp"
    
    # Process the file line by line
    awk '
    /Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier|aptos-labs\/aptos|aptos_moving_average|aptos_indexer_processor_sdk/ { 
        # Keep these lines unchanged
        print $0 
    }
    !/Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier|aptos-labs\/aptos|aptos_moving_average|aptos_indexer_processor_sdk/ { 
        # Replace in all other lines
        gsub(/Lumio/, "Lumio")
        gsub(/LUMIO/, "LUMIO")  
        gsub(/lumio/, "lumio")
        print $0
    }' "$file" > "$tmp_file"
    
    # Only replace if the file changed
    if ! cmp -s "$file" "$tmp_file"; then
        mv "$tmp_file" "$file"
        echo "Updated: $file"
    else
        rm "$tmp_file"
    fi
}

export -f replace_in_file

# Process all relevant file types
echo "Processing YAML files..."
find . -type f -name "*.yaml" -o -name "*.yml" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing JSON files..."
find . -type f -name "*.json" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing Move files..."
find . -type f -name "*.move" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing Markdown files..."
find . -type f -name "*.md" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing Python files..."
find . -type f -name "*.py" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing Shell scripts..."
find . -type f -name "*.sh" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing remaining Rust files..."
find . -type f -name "*.rs" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Processing remaining TOML files..."
find . -type f -name "*.toml" 2>/dev/null | grep -v target/ | grep -v .git/ | while read -r file; do
    replace_in_file "$file"
done

echo "Complete rebranding finished!"
