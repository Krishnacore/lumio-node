#!/bin/bash

echo "Batch Aptos to Lumio rebranding"
echo "================================"

# Function to process files in a directory
process_directory() {
    local dir="$1"
    local count=0

    echo "Processing $dir..."

    # Find text files in this directory
    find "$dir" -type f -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.yaml" -o -name "*.yml" -o -name "*.json" -o -name "*.move" -o -name "*.sh" -o -name "*.txt" 2>/dev/null | while read -r file; do
        # Skip if path contains .git or target
        if [[ "$file" == *"/.git/"* ]] || [[ "$file" == *"/target/"* ]]; then
            continue
        fi

        # Create temp file
        temp_file=$(mktemp)

        # Process with awk to preserve copyright
        awk '
        /Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier|Licensed under/ {
            # Keep these lines unchanged
            print $0
        }
        !/Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier|Licensed under/ {
            # Replace Aptos variants in other lines
            gsub(/Aptos/, "Lumio")
            gsub(/APTOS/, "LUMIO")
            gsub(/aptos/, "lumio")
            print $0
        }
        ' "$file" > "$temp_file" 2>/dev/null

        # Check if file changed and update
        if [ -s "$temp_file" ] && ! cmp -s "$file" "$temp_file" 2>/dev/null; then
            mv "$temp_file" "$file"
            ((count++))
        else
            rm -f "$temp_file" 2>/dev/null
        fi
    done

    echo "  Processed $count files in $dir"
}

# Process main directories
for dir in crates api aptos-move consensus config dkg docker ecosystem execution experimental keyless mempool network protos sdk scripts secure state-sync storage terraform testsuite types vm-validator peer-monitoring-service; do
    if [ -d "$dir" ]; then
        process_directory "$dir"
    fi
done

# Process root level files
echo "Processing root level files..."
for file in *.toml *.md *.json *.yaml *.yml *.txt; do
    if [ -f "$file" ] && [ "$file" != "LICENSE" ]; then
        temp_file=$(mktemp)

        awk '
        /Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier|Licensed under/ {
            print $0
        }
        !/Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier|Licensed under/ {
            gsub(/Aptos/, "Lumio")
            gsub(/APTOS/, "LUMIO")
            gsub(/aptos/, "lumio")
            print $0
        }
        ' "$file" > "$temp_file" 2>/dev/null

        if [ -s "$temp_file" ] && ! cmp -s "$file" "$temp_file" 2>/dev/null; then
            mv "$temp_file" "$file"
            echo "  Updated: $file"
        else
            rm -f "$temp_file" 2>/dev/null
        fi
    fi
done

echo ""
echo "================================"
echo "Rebranding complete!"
echo ""
echo "Next steps:"
echo "1. Review changes: git diff --stat"
echo "2. Check for issues: git status"
echo "3. Test build: cargo build"
echo "================================"