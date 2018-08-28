set -euxo pipefail

main() {
    if [ $TARGET = docs ]; then
        mkdir mdcheck
        curl https://raw.githubusercontent.com/korken89/mdcheckr/master/mdcheckr -o mdcheck/mdcheckr
        chmod +x mdcheck/mdcheckr
    elif [ $TARGET != x86_64-unknown-linux-gnu ]; then
        rustup target add $TARGET
    fi

}

main
