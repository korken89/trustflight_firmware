set -euxo pipefail

main() {
    if [ $TARGET = docs ]; then
        mkdir mdcheck
        curl https://raw.githubusercontent.com/korken89/mdcheckr/master/mdcheckr -o mdcheck/mdcheckr
        chmod +x mdcheck/mdcheckr
    else
        rustup target add thumbv7em-none-eabihf
    fi

}

main
