```
cd uniffi-bindgen
cargo build --release

cd shared_lib 
cargo build --release
```

To generate python bindings:
```
cargo run -p uniffi-bindgen -- generate --library ../target/debug/libsharedlib.dylib --language python --out-dir out
```

To generate typescript bindings:
```
cd shared_lib_web
wasm-pack build --target nodejs
```

In the python:
Copy the generated `.dylib` and `.py` files from the `out` directory.
import the generated python code
```
import sdk

print(sdk.add(10, 1))
print(sdk.create_ed25519())
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
