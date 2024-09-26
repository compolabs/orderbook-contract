forc clean
rm -rf ./spark-market-sdk/spark-market/out 
rm -rf ./spark-registry-sdk/spark-registry/out 
forc build --release
cp -r ./spark-market/out ./spark-market-sdk/spark-market/
cp -r ./spark-registry/out ./spark-registry-sdk/spark-registry/

cargo clean
cargo build