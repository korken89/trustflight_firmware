set -euxo pipefail

main() {
    if [ $TARGET = docs ]; then
        # Check all docs
        find . -name '*.md' -print0 | xargs -0 mdcheckr
    else
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
    fi
}

main
