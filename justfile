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

