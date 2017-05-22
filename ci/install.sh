set -ex

main() {
    if [ $TARGET != x86_64-unknown-linux-gnu ]; then
        cargo install xargo || true
        rustup component add rust-src
    fi
}

main
