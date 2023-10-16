# Rust WASM Seam Carving

## Intro

This repo a demo of the Seam Carving algorithm implemented in Rust running on Web Assembly. For a detailed write up, [read here](https://www.yanglinzhao.com/posts/seam-carving/).

## Running Locally

If you want to run the repo locally, you'll need the following:

```sh
node
yarn 
cargo
```

First, install the application dependencies with:

```sh
yarn install
```

Next, install the required `cargo watch` binary.

```sh
cargo install cargo-watch
```

Finally, run the development server with:

```
yarn dev
```

The application will be available on `http://localhost:3000`.
