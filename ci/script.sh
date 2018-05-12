set -ex

main() {
    local svd=STMicro/STM32F103xx.svd

    cargo check --target $TARGET
    cargo check --target $TARGET --features rt

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        # check than the patch can be applied to the original SVD
        local url=https://github.com/posborne/cmsis-svd/raw/python-0.4/data/$svd
        local td=$(mktemp -d)
        local svd=$(basename $svd)
        local svdraw=${svd}.raw
        local patch="${svd%.*}.patch"
        curl -L -o $svdraw $url
        dos2unix $svdraw
        cp $svdraw $svd
        patch -p1 $svd < $patch

        rm -rf $td
    fi
}

main
