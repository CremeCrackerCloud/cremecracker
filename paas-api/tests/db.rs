use paas_api::db;
use sqlx::sqlite::SqlitePool;
use std::env;
use tempfile::NamedTempFile;

mod memory_db_tests {
    use super::*;

    #[sqlx::test]
    async fn test_create_pool() {
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        let pool = db::create_pool().await;
        assert!(pool.is_ok());

        let pool = pool.unwrap();
        let result = sqlx::query("SELECT 1").execute(&pool).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_init_db() {
        let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();
        let result = db::init_db(&pool).await;
        assert!(result.is_ok());

        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
            .execute(&pool)
            .await;
        assert!(result.is_ok());
    }
}

mod file_db_tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[sqlx::test]
    async fn test_create_pool() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().display());
        env::set_var("DATABASE_URL", &db_url);
        let pool = db::create_pool().await;
        assert!(pool.is_ok());

        let pool = pool.unwrap();
        let result = sqlx::query("SELECT 1").execute(&pool).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_init_db() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().display());
        let pool = SqlitePool::connect(&db_url).await.unwrap();
        let result = db::init_db(&pool).await;
        assert!(result.is_ok());

        let result = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
            .execute(&pool)
            .await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_create_pool_nonexistent_dir() {
        env::set_var("DATABASE_URL", "sqlite:/nonexistent/dir/db.sqlite");
        let pool = db::create_pool().await;
        assert!(pool.is_err());
    }

    #[sqlx::test]
    async fn test_create_pool_readonly_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("db.sqlite");

        std::fs::create_dir_all(temp_dir.path()).unwrap();
        let mode = std::fs::Permissions::from_mode(0o444); // somente leitura
        std::fs::set_permissions(temp_dir.path(), mode).unwrap();

        env::set_var("DATABASE_URL", format!("sqlite:{}", db_path.display()));
        let pool = db::create_pool().await;
        assert!(pool.is_err());
    }
}

mod error_tests {
    use super::*;

    #[sqlx::test]
    async fn test_create_pool_invalid_url() {
        env::set_var("DATABASE_URL", "postgres://localhost:5432/db");
        let pool = db::create_pool().await;
        assert!(pool.is_err());

        env::set_var("DATABASE_URL", "not-a-url");
        let pool = db::create_pool().await;
        assert!(pool.is_err());

        env::set_var("DATABASE_URL", "");
        let pool = db::create_pool().await;
        assert!(pool.is_err());
    }
}
