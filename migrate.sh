#!/bin/bash

# Ensure the migrations directory exists
mkdir -p migrations

# Get the migration name from user input
if [ -z "$1" ]; then
    echo "Usage: $0 <migration_name>"
    exit 1
fi

# Find the latest migration number and increment it
latest=$(ls migrations | grep -E '^[0-9]+' | sort -n | tail -1 | cut -d'_' -f1)
if [ -z "$latest" ]; then
    next="001"
else
    next=$(printf "%03d" $((10#$latest + 1)))
fi

# Create migration files
up_file="migrations/${next}_$1.up.sql"
down_file="migrations/${next}_$1.down.sql"

touch "$up_file" "$down_file"

echo "Created migration files:"
echo "  - $up_file"
echo "  - $down_file"
