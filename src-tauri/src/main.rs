#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::params;

// Global connection manager pool
lazy_static! {
    static ref DB_POOL: Pool<SqliteConnectionManager> =
      r2d2::Pool::new(
        SqliteConnectionManager::file("mnote.db")
      ).unwrap();
}

fn main() {
    init();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running application");
}

fn init() {
    // init database
    let _ = DB_POOL.get()
        .unwrap()
        .execute("CREATE TABLE IF NOT EXISTS store (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  content TEXT NOT NULL,
                  tags TEXT
               );", params![])
        .unwrap();
}

struct _Store {
    id: u32,
    content: String,
    tags: String
}

// Exposed command
// TODO append to tauri::Builder

#[tauri::command]
fn _get_store() {

}

#[tauri::command]
fn _query_store() {

}

#[tauri::command]
fn _delete_store() {

}

#[tauri::command]
fn _save_store() {

}

#[tauri::command]
fn _update_store() {

}
