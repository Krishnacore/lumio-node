#!/bin/bash

echo "Starting Aptos to Lumio rebranding..."
echo "This script will:"
echo "  - Replace Aptos with Lumio (case-sensitive)"
echo "  - PRESERVE all copyright headers and license identifiers"
echo ""

# Counter for changed files
CHANGED_FILES=0
TOTAL_REPLACEMENTS=0

# Function to process a file
process_file() {
    local file="$1"
    local temp_file=$(mktemp)
    local replacements=0

    # Process line by line
    while IFS= read -r line; do
        # Check if this line contains copyright or license information
        if echo "$line" | grep -qE "(Copyright|©|SPDX-License-Identifier|Licensed under)"; then
            # Keep these lines unchanged
            echo "$line"
        else
            # Replace Aptos variants in other lines
            local new_line
            new_line=$(echo "$line" | sed -e 's/Aptos/Lumio/g' -e 's/APTOS/LUMIO/g' -e 's/aptos/lumio/g')

            # Count replacements
            if [ "$line" != "$new_line" ]; then
                ((replacements++))
            fi

            echo "$new_line"
        fi
    done < "$file" > "$temp_file"

    # Check if file actually changed
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        ((CHANGED_FILES++))
        ((TOTAL_REPLACEMENTS+=replacements))
        echo "✓ Updated: $file ($replacements replacements)"
        return 0
    else
        rm "$temp_file"
        return 1
    fi
}

echo "Searching for files to process..."

# Find all text files, excluding:
# - .git directory
# - LICENSE files
# - target directory (build artifacts)
# - Binary files
find . -type f \
  -not -path "./.git/*" \
  -not -path "./target/*" \
  -not -name "LICENSE*" \
  -not -name "*.png" \
  -not -name "*.jpg" \
  -not -name "*.jpeg" \
  -not -name "*.gif" \
  -not -name "*.ico" \
  -not -name "*.bin" \
  -not -name "*.wasm" \
  -not -name "*.jar" \
  -not -name "*.tar" \
  -not -name "*.gz" \
  -not -name "*.zip" \
  -not -name "*.pdf" \
  | while read -r file; do

    # Skip binary files
    if ! file "$file" | grep -qE "text|ASCII|UTF|source|script"; then
        continue
    fi

    # Check if file contains any form of Aptos (excluding copyright lines)
    if grep -vE "(Copyright|©|SPDX-License-Identifier)" "$file" 2>/dev/null | grep -qE "(Aptos|APTOS|aptos)"; then
        process_file "$file"
    fi
done

echo ""
echo "========================================="
echo "Rebranding complete!"
echo "Files changed: $CHANGED_FILES"
echo "Total replacements: $TOTAL_REPLACEMENTS"
echo ""
echo "Preserved:"
echo "  ✓ Copyright headers (© Aptos Foundation)"
echo "  ✓ SPDX-License-Identifiers"
echo "  ✓ LICENSE files"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff --stat"
echo "  2. Check specific changes: git diff"
echo "  3. Test compilation: cargo build"
echo "========================================="