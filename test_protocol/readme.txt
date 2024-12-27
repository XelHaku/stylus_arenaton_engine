./nitro-devnode/run-dev-node.sh 
cargo clean

cargo stylus export-abi

cargo stylus check
