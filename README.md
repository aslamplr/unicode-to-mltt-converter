Malayalam unicode to mltt converter

<div align="center">

  <h1>Malayalam unicode to mltt converter</h1>

  <strong>Malayalam unicode to mltt converter or encoder is a character encoder from Malayalam unicode characters to corresponding ML-TT character encodings. Using ML-TTKarthika character mapping for the task.</strong>

  
  <sub>Built with Rust 🦀 and WebAssembly🕸</sub>
</div>

## 🚴 Usage

### 🌎 Web example

Requires bundler such as webpack with appropriate wasm loaders to consume the library.
See the example in `www` directory for complete web application.

```javascript
import * as mlttConverter from 'unicode-to-mltt-converter';

fetch("www/public/karthika.map").then((resp) => resp.text()).then((defaultCharmapVal) => {
    mlttConverter.convert(inputValue, defaultCharmapVal);
});

```

### 🖥 Run Rust 🦀 CLI util

```
cargo run -- www/public/karthika.map "പശ്ചിമഘട്ടത്തിലെ ചുരുക്കം സ്ഥലങ്ങളിൽ മാത്രം വിരളമായി കാണപ്പെടുന്ന ഒരിനം പൂമ്പാറ്റയാണ് മലബാർ മിന്നൻ" 
```

Outputs:
`]ÝnaL«¯nse Npcp¡w Øe§fnÂ am{Xw hncfambn ImWs¸Sp¶ Hcn\w ]q¼mäbmWv ae_mÀ an¶³`

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Run all unit and integration tests

```
cargo test
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

## 🕸 Web Application using React is in the `www` directory inside this repo. Separate `README.md` included for `www`.


## Development dependencies

- rustc 1.41.1
- wasm-pack 0.9.1
- node v13.7.0
