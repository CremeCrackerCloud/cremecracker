# PAAS API

The PAAS (Platform as a Service) API is a Rust-based backend service that provides authentication and application management functionality.

## Features

- RESTful API endpoints using Actix-web
- SQLite database integration with SQLx
- JWT-based authentication
- OAuth2 support for multiple providers
- Database migrations
- Comprehensive test suite
- Environment-based configuration

## Tech Stack

- **Framework**: Actix-web 
- **Database**: SQLite with SQLx
- **Authentication**: OAuth2
- **Other Key Dependencies**:
  - `serde`: For serialization/deserialization
  - `jsonwebtoken`: For JWT handling
  - `chrono`: For datetime operations
  - `actix-session`: For session management

## Development Setup

1. Install dependencies:
   - Rust (2021 edition)
   - SQLite
   - SQLx CLI (`cargo install sqlx-cli`)

2. Set up environment:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. Run database migrations:
   ```bash
   sqlx migrate run
   ```

4. Start the development server:
   ```bash
   cargo run
   ```

## Testing

Run the test suite:
```bash
cargo test
```

For integration tests, a separate test database is used (configured via `tests.env`).

## Database Migrations

The project uses SQLx migrations to manage database schema changes. All migrations are stored in the `migrations/` directory.

### Migration Structure

Each migration consists of two SQL files:
- `[timestamp]_name.up.sql`: Contains the SQL to apply the migration
- `[timestamp]_name.down.sql`: Contains the SQL to revert the migration

Example:
```
migrations/
├── 20231226000000_init.up.sql   # Initial schema creation
└── 20231226000000_init.down.sql # Schema rollback
```

### Managing Migrations

SQLx CLI provides several commands for managing migrations:

```bash
# Create a new migration
sqlx migrate add -r <name>
# Example: sqlx migrate add -r create_users_table

# Run pending migrations
sqlx migrate run

# Revert the last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

### Migration Best Practices

1. **Atomic Changes**: Each migration should represent a single, atomic change to the database schema.
2. **Reversible**: Always provide both `up.sql` and `down.sql` files to allow rollbacks.
3. **Idempotent**: Migrations should be idempotent where possible (can be run multiple times without side effects).
4. **Testing**: Test both the up and down migrations before committing.
5. **Documentation**: Include comments in SQL files explaining complex changes.

### Development Workflow

1. Create a new migration:
   ```bash
   sqlx migrate add -r <descriptive_name>
   ```

2. Edit the generated files:
   - `up.sql`: Add your schema changes
   - `down.sql`: Add the reverse operations

3. Test the migration:
   ```bash
   # Apply the migration
   sqlx migrate run
   
   # Verify the changes
   # Run your tests or inspect the database
   
   # Optionally test the rollback
   sqlx migrate revert
   ```

4. Commit the migration files

### Automatic Migrations

The application automatically runs pending migrations on startup. This behavior can be controlled via environment variables:

- `AUTO_MIGRATE=true` (default): Run migrations on startup
- `AUTO_MIGRATE=false`: Skip automatic migrations

## Project Structure

```
paas-api/
├── src/
│   ├── auth.rs       # Authentication logic
│   ├── config.rs     # Configuration management
│   ├── db.rs         # Database connections and utilities
│   ├── error.rs      # Error handling
│   ├── handlers.rs   # Request handlers
│   ├── lib.rs        # Library exports
│   ├── main.rs       # Application entry point
│   ├── models.rs     # Data models
│   ├── routes.rs     # API route definitions
│   └── tests.rs      # Integration tests
├── migrations/       # Database migrations
└── tests/           # Test utilities and fixtures
```

## API Documentation

The API provides endpoints for:
- Authentication (OAuth2)
- User management
- Application deployment
- Resource management

For detailed API documentation, see the API reference in the root README.

## Environment Variables

Key environment variables:
- `DATABASE_URL`: SQLite database connection string
- `JWT_SECRET`: Secret for JWT token signing
- `OAUTH_*`: OAuth provider configurations
- `RUST_LOG`: Logging level configuration

See `.env.example` for a complete list of required variables.
