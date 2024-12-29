# PAAS UI

The PAAS (Platform as a Service) UI is a modern web interface built with Leptos, a Rust-based frontend framework. It provides a seamless user experience for managing applications and services.

## Tech Stack

- **Framework**: Leptos (Rust-based frontend framework)
- **Styling**: TailwindCSS
- **Build Tool**: Trunk (WASM bundler)
- **Key Dependencies**:
  - `leptos_router`: For client-side routing
  - `web-sys`: Web APIs bindings
  - `wasm-bindgen`: WebAssembly bindings
  - `gloo`: Browser APIs utilities

## Project Structure

```
paas-ui/
├── src/              # Source code
│   ├── api/           # API communication
│   │   ├── auth.rs    # Authentication API
│   │   └── mod.rs
│   ├── components/    # Reusable UI components
│   │   ├── nav.rs     # Navigation bar
│   │   ├── loading.rs # Loading spinner
│   │   └── mod.rs
│   ├── pages/         # Application pages
│   │   ├── home.rs    # Home page
│   │   ├── login.rs   # Login page
│   │   └── mod.rs
│   ├── app.rs         # Main application component
│   ├── lib.rs         # Library root
│   └── main.rs        # Application entry point
├── styles/           # CSS and Tailwind styles
│   ├── input.css      # Tailwind CSS entry point
│   └── output.css     # Generated CSS (don't edit)
├── public/           # Static assets
│   └── favicon.ico
├── dist/             # Build output
├── index.html        # HTML entry point
├── Cargo.toml         # Rust dependencies
├── package.json       # Node.js dependencies
├── tailwind.config.js # Tailwind configuration
├── postcss.config.js  # PostCSS configuration
├── Trunk.toml        # Trunk configuration
└── README.md          # This file
```

## Features

- Single Page Application (SPA) architecture
- Client-side routing
- Modern, responsive UI
- TailwindCSS for styling
- WebAssembly-powered performance
- 🔐 OAuth authentication with multiple providers:
  - GitHub
  - GitLab
  - Bitbucket
- 📱 Responsive design with Tailwind CSS
- ⚡ Fast, compiled WebAssembly
- 🔄 Client-side routing
- 🌐 Type-safe API communication
- 🎨 Modern, clean UI

## Prerequisites

Before you begin, ensure you have installed:

- Rust (latest stable)
- Node.js and npm (for Tailwind CSS)
- Trunk (`cargo install trunk`)
- wasm32 target (`rustup target add wasm32-unknown-unknown`)

## Setup

1. Install Rust and Cargo
2. Install Trunk:
   ```bash
   cargo install trunk
   ```
3. Install wasm32 target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
4. Install Node.js dependencies:
   ```bash
   npm install
   ```

## Development

- **Start development server**:
  ```bash
  trunk serve
  ```
- **Build for production**:
  ```bash
  trunk build --release
  ```

## Project Configuration

### Trunk.toml
Configures the WASM build process and development server settings.

### Tailwind
- `tailwind.config.js`: Tailwind configuration
- `postcss.config.js`: PostCSS plugins configuration

## Build Features

The project supports different build features:
- `csr`: Client-side rendering (default)
- `hydrate`: Hydration capabilities
- `ssr`: Server-side rendering support

## Browser Compatibility

The application is compiled to WebAssembly and supports all modern browsers that implement the WebAssembly specification.

## Configuration Files

### Trunk.toml
```toml
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "styles", "index.html"]

[serve]
address = "127.0.0.1"
port = 8080
```

### tailwind.config.js
```javascript
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
}
```

## Development

1. Clone the repository:
```bash
git clone <repository-url>
cd paas/paas-ui
```

2. Install dependencies:
```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies
npm install
```

3. Build Tailwind CSS:
```bash
# Build once
npm run build:css

# Or watch for changes
npm run watch:css
```

4. Run the development server:
```bash
trunk serve
```

The application will be available at `http://localhost:8080`. Trunk will automatically:
- Compile your Rust code to WebAssembly
- Bundle your assets
- Watch for changes and hot-reload

For the best development experience, run these commands in separate terminals:
```bash
# Terminal 1: Watch and rebuild CSS
npm run watch:css

# Terminal 2: Run development server
trunk serve
```

## Building for Production

To create a production build:

```bash
# Build CSS
npm run build:css

# Build the application
trunk build --release
```

The built files will be in the `dist` directory.

## Available Scripts

### Rust Commands
- `cargo build` - Build the project
- `cargo test` - Run tests
- `cargo fmt` - Format code
- `cargo clippy` - Run linter

### Node.js Commands
- `npm run build:css` - Build Tailwind CSS
- `npm run watch:css` - Watch and rebuild CSS

### Trunk Commands
- `trunk serve` - Start development server
- `trunk build` - Create development build
- `trunk build --release` - Create production build

## Environment Variables

The application expects the following environment variables:

- `API_BASE_URL` - Backend API URL (default: "http://localhost:3000/api")

## API Integration

The `api` module handles all communication with the backend. Each API endpoint is typed and returns `Result` types for proper error handling.

Example usage:
```rust
use crate::api::auth::AuthApi;

// Authenticate with GitHub
match AuthApi::github_auth().await {
    Ok(url) => // Handle success
    Err(e) => // Handle error
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Testing

Run the test suite:

```bash
cargo test
```

## Code Style

This project follows the Rust standard code style. To ensure your code is properly formatted:

```bash
cargo fmt
```

To check for common issues:

```bash
cargo clippy
```

## License

[Add your license information here]

## Acknowledgments

- [Leptos Documentation](https://leptos-rs.github.io/leptos/)
- [Rust WebAssembly Working Group](https://rustwasm.github.io/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Trunk](https://trunkrs.dev/)
