# DimSafe

> DimSafe is a calculation language with native units, automatic conversions, and dimensional checking, featuring a dedicated parser and a web-based editor.

## Build and Run

This project is a web application using Leptos in a WASM environment, so you need to build it for the web and run it in a web browser.

### Prerequisites

So first, you need to have cargo with wasm-pack and trunk installed. All dependencies are managed with cargo during the build process.

### Run

In development mode, you can run the server with:

```bash
trunk serve
```

This will start the server at http://localhost:8080 and automatically reload the page when you make changes.

### Build

If you want to get a production build, you can build with trunk:

```bash
trunk build --release
```

This will create a `dist` directory with the production of a static website.
You can then serve the website with a simple HTTP server like live-server extension in VS Code.

## Project Structure

- `src/`: source code in rust
- `public/`: css styles and html template
- `dist/`: production build

## License

Licensed under the Apache License, Version 2.0. See the `LICENSE` file for full terms.

### Apache 2.0 License Notice

```
Copyright 2025 Evan SERAY

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
