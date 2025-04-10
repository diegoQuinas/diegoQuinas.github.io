# diegoquinas.com

Personal portfolio site - Backend & Systems Engineer (Go / Rust / AI).

Live at **[diegoquinas.com](https://diegoquinas.com)**.

## Stack

- Built with [Dioxus](https://dioxuslabs.com) 0.7 (Rust) + SSR
- Tailwind CSS v4
- WASM hydration for interactivity
- Hosted on GitHub Pages

## Local preview

```bash
cd ~/personal/diegoquinas-rs
make tailwind && dx serve
```

## Build

```bash
cd ~/personal/diegoquinas-rs
make all
```

Generated static files are copied into this repo for deployment.

## Deploy

Pushes to the default branch are published automatically by GitHub Pages.
