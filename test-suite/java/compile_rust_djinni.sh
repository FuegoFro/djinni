

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
RUST_ROOT="$DIR/../handwritten-src/rust"

pushd "$RUST_ROOT" > /dev/null

cargo build --lib

popd > /dev/null

cp "$RUST_ROOT/target/debug/libdjinni_test_suite.dylib" $1
