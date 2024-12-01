#!/bin/sh

# Set the default log level to 'info'
log_level=${2:-info}

# Set the RUST_LOG environment variable
export RUST_LOG=$log_level

# Get the day number from the command-line arguments
day_number=$1

# Set the default part number to 1
part_number=${3:-all}

# Set the default input number to 0
input_number=${4:-all}

# Parse the command-line arguments
while [ $# -gt 0 ]; do
  case "$1" in
    -p|--part)
      shift
      part_number=$1
      ;;
    -t|--input)
      shift
      input_number=$1
      ;;
  esac
  shift
done

# Run the program with the specified options
cargo run --release --bin $day_number -- -p $part_number -t $input_number