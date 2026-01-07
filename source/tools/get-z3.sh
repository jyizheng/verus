#! /bin/bash -eu

# Script to get z3 from the local z3 directory in the repository
# The z3 directory contains Z3 version 4.12.5 x64-glibc-2.31

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Repository root is two levels up from source/tools
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
Z3_LOCAL_DIR="$REPO_ROOT/z3"

# delete the existing z3 because of caching issue on macs
rm -f z3
rm -f z3.exe

# Check if local z3 directory exists
if [ -d "$Z3_LOCAL_DIR" ]; then
    # Copy from local z3 directory
    if [ -f "$Z3_LOCAL_DIR/bin/z3" ]; then
        cp "$Z3_LOCAL_DIR/bin/z3" .
        echo "z3 copied from local directory: $Z3_LOCAL_DIR"
        echo "z3 located at $(pwd)/z3"
    elif [ -f "$Z3_LOCAL_DIR/bin/z3.exe" ]; then
        cp "$Z3_LOCAL_DIR/bin/z3.exe" .
        echo "z3.exe copied from local directory: $Z3_LOCAL_DIR"
        echo "z3.exe located at $(pwd)/z3.exe"
    else
        echo "Error: z3 binary not found in $Z3_LOCAL_DIR/bin/"
        exit 1
    fi
else
    echo "Error: Local z3 directory not found at $Z3_LOCAL_DIR"
    exit 1
fi
