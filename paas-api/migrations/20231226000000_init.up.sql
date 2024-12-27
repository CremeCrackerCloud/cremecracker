-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL,  -- OAuth provider (github, gitlab, bitbucket)
    provider_user_id TEXT NOT NULL,  -- User ID from the OAuth provider
    username TEXT NOT NULL,
    email TEXT,
    avatar_url TEXT,
    access_token TEXT,
    refresh_token TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(provider, provider_user_id)  -- Ensure unique provider + provider_user_id combination
);

-- Create git_providers table for managing multiple provider connections per user
CREATE TABLE IF NOT EXISTS git_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    provider_type TEXT NOT NULL,  -- "github", "gitlab", "bitbucket"
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    expires_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(user_id, provider_type)  -- One connection per provider type per user
);

-- Create repositories table
CREATE TABLE IF NOT EXISTS repositories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    provider_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    url TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (provider_id) REFERENCES git_providers(id) ON DELETE CASCADE
);
