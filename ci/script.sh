set -euxo pipefail

main() {
    # Testing firmware
    cd firmware
    cargo check --target $TARGET
    cargo test
    cd ..

    # Testing board
    cd board
    cargo check --target $TARGET
    cargo test
    cd ..

    # Check binary
    cd binary
    cargo check --target $TARGET
    cd ..
}

main
