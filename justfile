prepare year day:
    cargo run -p advent_of_code_scripts -- --year {{year}} --day {{day}} --cwd {{justfile_directory()}}

prepare-year year:
    for day in {1..25}; do \
        just prepare {{year}} $day; \
    done

prepare-all:
    for year in {2015..2025}; do \
        just prepare-year $year; \
    done

rebuild:
    cd advent_of_code_rust && \
    cargo clean && \
    cargo build

rust-r year day:
    cargo run --manifest-path advent_of_code_rust/Cargo.toml \
    -p advent_of_code_{{year}} {{day}}

rust-t year day:
    cargo test --manifest-path advent_of_code_rust/Cargo.toml \
    -p advent_of_code_{{year}} day{{day}}