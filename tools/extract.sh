rm -rf modules/
mkdir modules/

cd extractor/
cargo run -- \
    -r RoactCompat-9c8468d8-8a7220fd \
    -r Throat \
    -e ../modules
