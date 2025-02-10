A personal portfolio site and blog. Built in Rust using the Axum web framework, Askama HTML templating, and Tailwind CSS styles.

![Portfolio screenshot](https://raw.githubusercontent.com/joshua-smart/portfolio/refs/heads/main/screenshot.png)

# Building

```sh
cargo build -r
```

# Usage

```
Usage: portfolio [OPTIONS]

Options:
      --log-level <LOG_LEVEL>  [env: LOG_LEVEL=] [default: INFO]
      --asset-dir <ASSET_DIR>  Directory to serve static assets from [env: ASSET_DIR=] [default: assets/]
  -p, --port <PORT>            [env: PORT=] [default: 3000]
  -a, --address <ADDRESS>      [env: ADDRESS=] [default: 0.0.0.0]
  -d, --data-path <DATA_PATH>  Path to a file containing static data in RON format [env: DATA_PATH=] [default: data.ron]
  -h, --help                   Print help
```
