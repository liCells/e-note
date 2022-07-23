#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::params;

fn main() {
  let db_pool = get_sqlite_pool();
  db_pool.get()
      .unwrap()
      .execute("CREATE TABLE IF NOT EXISTS store (
                  id INT PRIMARY KEY NOT NULL,
                  content TEXT NOT NULL,
                  tags TEXT
               );", params![])
      .unwrap();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running application");
}

fn get_sqlite_pool() -> Pool<SqliteConnectionManager> {
  let manager = SqliteConnectionManager::file("mnote.db");
  r2d2::Pool::new(manager).unwrap()
}
