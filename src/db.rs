use rusqlite::{Connection, Result, Error};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::fs;

// stocke l'instance unique de la db (singleton)
static DB: Lazy<Mutex<Result<DbManager>>> = Lazy::new(|| {
    if fs::create_dir_all("data").is_err() {
        return Mutex::new(Err(Error::ExecuteReturnedResults));
    }
    Mutex::new(DbManager::new())
});

pub struct DbManager {
    conn: Connection,
}

impl DbManager {
    fn new() -> Result<Self> {
        let conn = Connection::open("data/hub_gaming.db")?;
        Ok(Self { conn })
    }

    fn p_init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT UNIQUE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS guessing (
                id INTEGER PRIMARY KEY,
                user_id INTEGER,
                username TEXT,
                score INTEGER,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS motus (
                id INTEGER PRIMARY KEY,
                user_id INTEGER,
                username TEXT,
                score INTEGER,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS reflex (
                id INTEGER PRIMARY KEY,
                user_id INTEGER,
                username TEXT,
                score INTEGER,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            [],
        )?;

        Ok(())
    }

    fn p_create_user(&self, username: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO users (username) VALUES (?1)",
            [username],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }

    fn p_get_user_by_username(&self, username: &str) -> Result<Option<(i64, String)>> {
        let mut stmt = self.conn.prepare("SELECT id, username FROM users WHERE username = ?1")?;
        let mut rows = stmt.query([username])?;
        
        if let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            Ok(Some((id, name)))
        } else {
            Ok(None)
        }
    }

    fn p_add_game_score(&self, table_name: &str, user_id: i64, username: &str, score: i32) -> Result<()> {
        let query = format!(
            "INSERT INTO {} (user_id, username, score) VALUES (?1, ?2, ?3)",
            table_name
        );
        
        self.conn.execute(&query, [&user_id.to_string(), username, &score.to_string()])?;

        Ok(())
    }

    fn p_get_leaderboard(&self, table_name: &str) -> Result<Vec<(String, i32)>> {
        let query = format!(
            "SELECT username, score FROM {} ORDER BY score DESC LIMIT 5",
            table_name
        );
        
        let mut stmt = self.conn.prepare(&query)?;
        let results = stmt.query_map([], |row| {
            let username: String = row.get(0)?;
            let score: i32 = row.get(1)?;
            Ok((username, score))
        })?
        .collect::<Result<Vec<(String, i32)>>>()?;
            
        Ok(results)
    }

    fn p_get_user_best_scores(&self, user_id: i64) -> Result<Vec<(String, Option<i32>)>> {
        let games = vec!["guessing", "motus", "reflex"];
        let mut results = Vec::new();
        
        for game in games {
            let query = format!(
                "SELECT score FROM {} WHERE user_id = ?1 ORDER BY score DESC LIMIT 1",
                game
            );
            
            let mut stmt = self.conn.prepare(&query)?;
            let mut scores = stmt.query_map([user_id], |row| {
                let score: i32 = row.get(0)?;
                Ok(score)
            })?;
            
            if let Some(Ok(score)) = scores.next() {
                results.push((game.to_string(), Some(score)));
            } else {
                results.push((game.to_string(), None));
            }
        }
        
        Ok(results)
    }


    // API publique statique
    pub fn init() -> Result<()> {
        let db_guard = DB.lock().unwrap();
        match &*db_guard {
            Ok(manager) => manager.p_init_tables(),
            Err(_) => Err(Error::ExecuteReturnedResults),
        }
    }
    
    pub fn get_user_by_username(username: &str) -> Result<Option<(i64, String)>> {
        let db_guard = DB.lock().unwrap();
        match &*db_guard {
            Ok(manager) => manager.p_get_user_by_username(username),
            Err(_) => Err(Error::ExecuteReturnedResults),
        }
    }
    
    pub fn create_user(username: &str) -> Result<i64> {
        let db_guard = DB.lock().unwrap();
        match &*db_guard {
            Ok(manager) => manager.p_create_user(username),
            Err(_) => Err(Error::ExecuteReturnedResults),
        }
    }
    
    pub fn add_game_score(table_name: &str, user_id: i64, username:&str, score: i32) -> Result<()> {
        let db_guard = DB.lock().unwrap();
        match &*db_guard {
            Ok(manager) => manager.p_add_game_score(table_name, user_id,username, score),
            Err(_) => Err(Error::ExecuteReturnedResults),
        }
    }

    pub fn get_leaderboard(table_name: &str) -> Result<Vec<(String, i32)>> {
        let db_guard = DB.lock().unwrap();
        match &*db_guard {
            Ok(manager) => manager.p_get_leaderboard(table_name),
            Err(_) => Err(Error::ExecuteReturnedResults),
        }
    }

    pub fn get_user_best_scores(user_id: i64) -> Result<Vec<(String, Option<i32>)>> {
        let db_guard = DB.lock().unwrap();
        match &*db_guard {
            Ok(manager) => manager.p_get_user_best_scores(user_id),
            Err(_) => Err(Error::ExecuteReturnedResults),
        }
    }

}