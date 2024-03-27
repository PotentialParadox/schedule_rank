# Schedule Rank

## Description
Schedule Rank is a program that pairs residents with their preferred track

## Usage
To use Schedule Rank, follow these steps:

1. Use a `file.csv` with the format
| id | submission_date | 1st Choice, 2nd Choice, ... nth Choice |
2. use the command```paste -d ',' file.csv <(cat file.csv | schedule_rank) > file_ranked.csv```

results will be in `file_ranked.csv`

## Dependencies
MacOS or Linux

Rust

## Installation
To install Schedule Rank, follow these steps:

1. clone
2. `cd schedule_rank`
3. `cargo build --release`
4. ```export PATH=$PATH:`pwd`/target/release```