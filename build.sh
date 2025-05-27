#!/bin/bash

# Check if a file name was provided as an argument
if [ $# -eq 0 ]; then
    echo "Usage: $0 <source_file.rs>"
    exit 1
fi

# Assign the first command line parameter to a variable
SOURCE_FILE=$1

# I added this variable.
PARAM_1=$2

# Compile the Rust source file using rustc
rustc "$SOURCE_FILE"

# Check if the compilation was successful
if [ $? -eq 0 ]; then
    # Extract the executable name from the source file
    EXECUTABLE="${SOURCE_FILE%.*}"

    # Execute the compiled program and handle any errors
    if ./"$EXECUTABLE" $PARAM_1; then
        echo "Program executed successfully."
    else
        echo "Error executing the program."
        exit 1
    fi
else
    echo "Compilation failed."
    exit 1
fi