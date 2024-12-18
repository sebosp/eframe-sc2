# eframe sc2

[![dependency status](https://deps.rs/repo/github/sebosp/eframe-sc2/status.svg)](https://deps.rs/repo/github/sebosp/eframe-sc2)
[![Build Status](https://github.com/sebosp/eframe-sc2/workflows/CI/badge.svg)](https://github.com/sebosp/eframe-sc2/actions?workflow=CI)

This is a repo based on [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

In this repo there is a backend and a front-end.
The backend runs as web server with axum and tokio.
It receives requests to interact with the polars datasets and serves HTML.

The frontend may run as native or in the browser. It should interact with the backend.

## Running the backend (axum)

```
$ cargo watch -x clippy -x "run -- --source-dir $HOME/git/s2protocol-rs/ipcs/ -d -v debug"
```

This serves the front end as static files, the intention is to proxy the frontend as well so that it can avoid CORS issues.

## Running the frontend for development (trunk)

```
$ trunk serve --address 0.0.0.0 --proxy-insecure --proxy-backend http://$hostname:3000/ -v
```

This allows running requests from the frontend (wasm) to the backend over the same host:port, avoiding CORS related issues.
Proxied by trunk itself.
