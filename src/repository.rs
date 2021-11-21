use std::path::Path;

use rusqlite::Connection;

pub struct Repository;

impl Repository {
    pub fn create<P: AsRef<Path>>(filename: P) {
        let conn = Connection::open(filename.as_ref()).unwrap();

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
        .unwrap();
    }
}