# Pythonathon

Server for hosting coding challenges for learning Python. Built in Rust with Leptos and embedding a RustPython to run via WASM in-browser, this aims to provide an all-in-one package for hosting Python challenges.

## Deployment

To run the server locally at `http://localhost:3000`:

```bash
cargo leptos watch
```

To deploy the server on AWS Lambda:

```bash
cargo leptos build --release
cargo lambda build --release --features=ssr --no-default-features
cargo lambda deploy --include target/site --enable-function-url
```

Front-end and Python interpreter are built-in, just deploy and go to the site.
