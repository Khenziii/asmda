use crate::api_wrappers::APIWrapper;
use crate::environment;
use crate::logger;
use crate::utils::constants::{APIWrapperIdentificator, ArchiverIdentificator};
use crate::utils::time::{str_to_system_time, system_time_to_str};
use rusqlite::{Connection, Error::QueryReturnedNoRows};
use std::time::SystemTime;

pub struct DatabaseClient {
    connection: Connection,
}

impl APIWrapper for DatabaseClient {
    fn get_identificator(&self) -> APIWrapperIdentificator {
        APIWrapperIdentificator::Database
    }
}

impl DatabaseClient {
    pub fn new() -> Self {
        let config = environment::environment();

        let connection = Connection::open(&config.metadata.database_path).unwrap_or_else(|_| {
            panic!(
                "Failed to open `{}` database!",
                &config.metadata.database_path
            )
        });
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS schedule (
                id          INTEGER PRIMARY KEY,
                app_name    TEXT NOT NULL,
                next_run    TEXT NOT NULL
            )",
                [],
            )
            .expect("Failed to initialize database!");

        DatabaseClient { connection }
    }

    pub fn get_next_run_by_app_name(&self, app_name: ArchiverIdentificator) -> SystemTime {
        let next_run_query_result = self.connection.query_row(
            "SELECT next_run FROM schedule WHERE app_name = ?1",
            [app_name.as_str()],
            |row| row.get(0),
        );

        match next_run_query_result {
            Ok(next_run_string) => str_to_system_time(next_run_string),
            Err(QueryReturnedNoRows) => SystemTime::now(),
            Err(e) => panic!("Failed to get `next_run` timestamp: {e}"),
        }
    }

    pub fn update_next_run(&self, app_name: ArchiverIdentificator, new_next_run: SystemTime) {
        let new_next_run_string = system_time_to_str(new_next_run);
        let amount_of_changed_rows = self
            .connection
            .execute(
                "UPDATE schedule SET next_run = ?1 WHERE app_name = ?2",
                [new_next_run_string.clone(), app_name.as_str().to_string()],
            )
            .expect("Failed to update next_run!");

        logger::debug(&format!(
            "updated the {}'s next_run to {}",
            app_name.as_str(),
            new_next_run_string
        ));

        if amount_of_changed_rows == 0 {
            self.connection
                .execute(
                    "INSERT INTO schedule (app_name, next_run) VALUES (?1, ?2)",
                    [app_name.as_str().to_string(), new_next_run_string.clone()],
                )
                .expect("Failed to insert next_run!");
        }
    }
}
