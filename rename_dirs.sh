#!/bin/bash

echo "Renaming directories from aptos to lumio..."
echo "==========================================="

# Function to rename directory and update git
rename_dir() {
    local old_name="$1"
    local new_name="$2"

    if [ -d "$old_name" ]; then
        echo "Renaming: $old_name -> $new_name"
        git mv "$old_name" "$new_name" 2>/dev/null || mv "$old_name" "$new_name"
    fi
}

# Rename main directories
rename_dir "aptos-move" "lumio-move"
rename_dir "aptos-node" "lumio-node"

# Rename all crates directories
for dir in crates/aptos-*; do
    if [ -d "$dir" ]; then
        new_name=$(echo "$dir" | sed 's/aptos-/lumio-/')
        rename_dir "$dir" "$new_name"
    fi
done

# Rename subdirectories in lumio-move (formerly aptos-move)
if [ -d "lumio-move" ]; then
    for dir in lumio-move/aptos-*; do
        if [ -d "$dir" ]; then
            new_name=$(echo "$dir" | sed 's/aptos-/lumio-/')
            rename_dir "$dir" "$new_name"
        fi
    done
fi

# Count renamed directories
echo ""
echo "==========================================="
echo "Directory renaming complete!"
echo ""
echo "Next steps:"
echo "1. Review changes: git status"
echo "2. Update import paths in Cargo.toml files"
echo "3. Try building: cargo build"
echo "==========================================="