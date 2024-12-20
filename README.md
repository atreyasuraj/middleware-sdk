```
cd uniffi-bindgen
cargo build --release

cd shared_lib 
cargo build --release
cargo run -p uniffi-bindgen -- generate --library ../target/debug/libsharedlib.dylib --language python --out-dir out

cd shared_lib_web
wasm-pack build --target nodejs
```

In the typescript:

```import * as sdk from '/middleware-sdk/shared_lib_web/pkg/shared_lib_web'``` (Change to your path)

```
// test.ts
async function main() {
  let keys = sdk.create_ed25519();
    console.log(keys);

    let num = sdk.add(1n, 2n);
    console.log(num);
}
```

Run using command:

```
npx tsx test.ts
```
