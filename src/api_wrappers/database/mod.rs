use rusqlite::{Connection};
use std::time::{SystemTime};
use crate::utils::time::{str_to_system_time, system_time_to_str};
use crate::api_wrappers::APIWrapper;
use crate::utils::constants::{APIWrapperIdentificator, ArchiverIdentificator};

pub struct DatabaseClient {
    connection: Connection,
}

impl APIWrapper for DatabaseClient {
    fn get_identificator(&self) -> APIWrapperIdentificator {
        APIWrapperIdentificator::Database
    }
}

impl DatabaseClient {
    pub async fn new() -> Self {
        let connection = Connection::open("asmda.sqlite")
            .expect("Failed to open `asmda.sqlite` database!");
        connection.execute(
            "CREATE TABLE IF NOT EXISTS schedule (
                id          INTEGER PRIMARY KEY,
                app_name    TEXT NOT NULL,
                next_run    TEXT NOT NULL
            )",
            []
        ).expect("Failed to initialize database!");

        DatabaseClient { connection }
    }

    pub async fn get_next_run_by_app_name(
        &self,
        app_name: ArchiverIdentificator,
    ) -> SystemTime {
        let next_run_string = self.connection.query_row(
            "SELECT next_run FROM schedule WHERE app_name = ?1",
            [app_name.as_str()],
            |row| row.get(0),
        ).expect("Failed to get `next_run` timestamp!");
        let next_run = str_to_system_time(next_run_string);
        next_run
    }

    pub async fn update_next_run(
        &self,
        app_name: ArchiverIdentificator,
        new_next_run: SystemTime,
    ) {
        let new_next_run_string = system_time_to_str(new_next_run);
        self.connection.execute(
            "UPDATE schedule SET next_run = ?1 WHERE app_name = ?2",
            [new_next_run_string, app_name.as_str().to_string()],
        ).expect("Failed to update next_run!");
    }
}
