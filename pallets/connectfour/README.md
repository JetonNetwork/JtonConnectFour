License: Unlicense

### Run Benchmark for pallet-connectfour

```bash
./target/release/node-template benchmark --chain dev --execution wasm --wasm-execution compiled --pallet pallet_connectfour --extrinsic '*' --steps 20 --repeat 10 --raw --output .
```