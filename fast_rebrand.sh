#!/bin/bash

echo "Fast Aptos to Lumio rebranding using sed"
echo "========================================="
echo "This will:"
echo "  - Replace Aptos/APTOS/aptos with Lumio/LUMIO/lumio"
echo "  - PRESERVE copyright headers and license identifiers"
echo ""

# First, let's find all text files that need processing
echo "Finding files to process..."

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
  -not -name "rebrand*.sh" \
  -not -name "*.log" \
  | while read -r file; do
    # Skip binary files
    if file "$file" 2>/dev/null | grep -qE "text|ASCII|UTF|source|script"; then
        echo "$file"
    fi
done > files_to_process.txt

TOTAL_FILES=$(wc -l < files_to_process.txt)
echo "Found $TOTAL_FILES text files to process"

echo ""
echo "Processing files..."

# Process each file
PROCESSED=0
cat files_to_process.txt | while read -r file; do
    # Use sed with in-place editing, but preserve copyright lines
    # Create a temporary file first
    temp_file=$(mktemp)

    # Process the file
    awk '
    /Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier/ {
        # Keep copyright and license lines unchanged
        print $0
    }
    !/Copyright.*Aptos|©.*Aptos|SPDX-License-Identifier/ {
        # Replace Aptos variants in other lines
        gsub(/Aptos/, "Lumio")
        gsub(/APTOS/, "LUMIO")
        gsub(/aptos/, "lumio")
        print $0
    }
    ' "$file" > "$temp_file"

    # Only update if file changed
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        ((PROCESSED++))
        if [ $((PROCESSED % 100)) -eq 0 ]; then
            echo "Processed $PROCESSED/$TOTAL_FILES files..."
        fi
    else
        rm "$temp_file"
    fi
done

echo ""
echo "========================================="
echo "Rebranding complete!"
echo "Files processed: $PROCESSED out of $TOTAL_FILES"
echo ""
echo "Preserved:"
echo "  ✓ Copyright headers (© Aptos Foundation)"
echo "  ✓ SPDX-License-Identifiers"
echo "  ✓ LICENSE files"
echo ""
echo "Next steps:"
echo "  1. Review changes: git status"
echo "  2. Check specific changes: git diff --stat"
echo "  3. Test compilation: cargo build"
echo "========================================="

# Clean up
rm -f files_to_process.txt