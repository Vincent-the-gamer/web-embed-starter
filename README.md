# web-embed-starter

An Axum web application starter that embeds a web ui into your Rust-based web server binary file.

## Why

I like the design of [Dufs](https://github.com/sigoden/dufs) very much, it contains a Web UI into one single binary file.

## Prerequisites

You need to install the following dev environments

- Rust
- Node.js (v20+ is recommended)
- Git

## Usage

1. Clone the repository without `.git` folder, or directly use this template in GitHub.

```shell
# clone without .git folder
npx degit Vincent-the-gamer/web-embed-starter
```

2. Install both Rust and Web UI(a Vue project) dependencies

Do these things sequentially to initialize the project.

```shell
cd webui

pnpm i

pnpm run build

cargo check
```

3. Write your backend APIs in Rust project
4. Write your Web UI in Vue project.
5. Build your frontend UI.
6. Set your host and port, then build your backend server into a binary file, it will contain the Web UI, visit it at the base url of your server.

# License

[MIT License](./LICENSE) - Copyright (c) 2026-present Vincent-the-gamer
