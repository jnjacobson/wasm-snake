# WASM Snake Game

A Snake game implementation using Rust and WebAssembly, running in the browser. Built with Vite, Tailwind CSS, and Rust WASM.



## Requirements

- Docker
- Docker Compose

## Development

1. Start the development container:
```bash
make up
```

2. Build the WASM module:
```bash
make build
```

3. The development server will be available at `http://localhost:5173` with hot reloading enabled.

## Project Structure

- `snake-game/` - Rust WASM source code
- `src/` - Frontend JavaScript code
- `index.html` - Main HTML entry point
