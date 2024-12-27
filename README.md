# PAAS (Platform as a Service)

A modern, secure Platform as a Service built with Rust, featuring a robust backend API and a WebAssembly-powered frontend.

## Project Overview

The project consists of two main components:

### 1. Backend (`paas-api`)
- RESTful API built with Actix-web
- SQLite database with SQLx
- JWT-based authentication
- OAuth2 support for multiple providers
- Database migrations
- Comprehensive test suite

### 2. Frontend (`paas-ui`)
- Built with Leptos framework
- WebAssembly-powered for high performance
- Tailwind CSS for modern styling
- Type-safe API communication
- Responsive design
- Client-side routing

## Project Structure

```
paas/
├── paas-api/          # Backend service
│   ├── src/           # Source code
│   ├── migrations/    # Database migrations
│   └── tests/         # Integration tests
├── paas-ui/           # Frontend application
│   ├── src/           # Source code
│   ├── styles/        # CSS styles
│   └── public/        # Static assets
└── data/             # Persistent data storage
```

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd paas
```

2. Set up the backend:
```bash
cd paas-api
cp .env.example .env  # Configure your environment
cargo install sqlx-cli
sqlx migrate run
cargo run
```

3. Set up the frontend:
```bash
cd ../paas-ui
npm install
npm run build:css
trunk serve
```

The application will be available at:
- Frontend: http://localhost:8080
- Backend API: http://localhost:3000

## Development

Each component has its own development workflow and documentation. See:
- [Backend Documentation](paas-api/README.md)
- [Frontend Documentation](paas-ui/README.md)

## Prerequisites

- Rust (2021 edition)
- SQLite
- Node.js and npm
- Trunk (`cargo install trunk`)
- wasm32 target (`rustup target add wasm32-unknown-unknown`)

## Testing

Run tests for all components:
```bash
# Backend tests
cd paas-api && cargo test

# Frontend tests
cd paas-ui && cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
