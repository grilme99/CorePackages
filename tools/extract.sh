rm -rf modules/
mkdir modules/

cd extractor/
cargo run -- \
    -r RoactCompat-9c8468d8-8a7220fd \
    -r React-a406e214-4230f473 \
    -r Throat \
    -r VirtualizedList \
    -e ../modules
