set -ex

main() {
    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        cargo check
    else
        xargo check --target $TARGET
    fi
}

main
