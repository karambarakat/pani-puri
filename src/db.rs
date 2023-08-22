use sqlx::postgres::PgPoolOptions;

pub async fn connect() -> sqlx::PgPool {
    let db_connection = std::env::var("POSTGRES_DB").expect("no database url");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection)
        .await
        .expect("no database connection");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            email           VARCHAR NOT NULL UNIQUE,
            password        VARCHAR NOT NULL,
            display_name    VARCHAR,
            avatar          VARCHAR,
            providers       VARCHAR NOT NULL,
            created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS category (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR NOT NULL,
            color       VARCHAR,
            icon        VARCHAR,
            created_by  integer NOT NULL,
            created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            
            FOREIGN KEY (created_by) REFERENCES users(id)
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS category_temp (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            color           VARCHAR,
            icon            VARCHAR,
            created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS entry (
            id          SERIAL PRIMARY KEY,
            title       VARCHAR NOT NULL,
            amount      NUMERIC NOT NULL,
            category    integer,
            created_by  integer NOT NULL,
            note        VARCHAR,
            created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (category) REFERENCES category(id),
            FOREIGN KEY (created_by) REFERENCES users(id)
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}
