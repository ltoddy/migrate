use std::path::Path;

use rusqlite::Connection;

pub struct Repository;

impl Repository {
    pub fn initialize<P: AsRef<Path>>(filename: P) {
        let conn = Connection::open(filename.as_ref()).expect("initialize sqlite");

        conn.execute(
            r#"
                CREATE TABLE IF NOT EXISTS `execution`
                (
                    `id`         INT AUTO_INCREMENT PRIMARY KEY,
                    `filename`   VARCHAR(255),
                    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )
            "#,
            [],
        )
        .expect("initialize migration manager failed");
    }
}
