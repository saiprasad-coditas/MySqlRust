use mysql::{prelude::Queryable, OptsBuilder, Pool, PooledConn};

pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new() -> Self {
        // Specify MySQL server connection parameters
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some("127.0.0.1"))
            .user(Some("SaiprasadGupta"))
            .pass(Some("Saiprasad@123"));

        // Create a MySQL pool without selecting a specific database
        let pool = Pool::new(opts.clone()).expect("Failed to create database connection pool");

        // Create a new connection to check if the database exists
        let mut conn = Pool::get_conn(&pool).expect("Couldn't get connection from the pool");

        // Specify the database name
        let db_name = "employeeprotal";

        // Check if the database exists
        if !Self::database_exists(&mut conn, db_name) {
            // If the database does not exist, create it
            conn.query_drop(format!("CREATE DATABASE IF NOT EXISTS {}", db_name))
                .expect("Failed to create database");
        }

        // Now, reconnect to the created or existing database
        let opts = OptsBuilder::from_opts(opts.db_name(Some(db_name)));
        let pool = Pool::new(opts).expect("Failed to create database connection pool");

        Database { pool }
    }

    // Database existence check is now a static method
    pub(crate) fn database_exists(conn: &mut PooledConn, db_name: &str) -> bool {
        let result: Result<(), _> = conn.query_drop(format!("USE {}", db_name));
        result.is_ok()
    }

    pub fn create_tables(&self) -> Result<(), String> {
        let mut conn = self.pool.get_conn().map_err(|e| format!("Couldn't get connection from the pool: {}", e))?;

        // Execute SQL queries to create tables if they do not exist
        conn.query_drop(
            "CREATE TABLE IF NOT EXISTS manager (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                role VARCHAR(255) NOT NULL
            )",
        ).map_err(|e| format!("Failed to create manager table: {}", e))?;

        conn.query_drop(
            "CREATE TABLE IF NOT EXISTS employee (
                id INT AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                role VARCHAR(255) NOT NULL,
                manager_id INT,
                FOREIGN KEY (manager_id) REFERENCES manager(id)
            )",
        ).map_err(|e| format!("Failed to create employee table: {}", e))?;

        println!("Tables created or already exist");
        Ok(())
    }
}
