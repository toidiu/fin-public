#![allow(dead_code, unused)]

use crate::data::*;
use chrono::prelude::*;
use postgres::{Connection, TlsMode};
use r2d2_postgres::{PostgresConnectionManager, TlsMode as R2TlsMode};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::sync::RwLock;
use std::sync::{Arc, Mutex};

const CLUSTER_URI: &'static str = "postgres://postgres@localhost:5432";

lazy_static! {
    static ref COUNTER: Arc<Mutex<u8>> = Arc::new(Mutex::new(1));
}
pub struct TestHelper {}

impl TestHelper {
    pub fn get_test_db(db_name: &str) -> PgFinDb {
        let r2 = {
            let manager = PostgresConnectionManager::new(
                Self::get_test_db_uri(db_name).as_str(),
                R2TlsMode::None,
            )
            .unwrap();
            r2d2::Pool::builder()
                .max_size(1)
                .build(manager)
                .expect("Failed to create pool")
        };

        PgFinDb {
            conn: r2.get().expect("expected db connection"),
            logger: slog::Logger::root(slog::Discard, o!("key" => "fake")),
        }
    }

    pub fn run_test_opt_teardown<T>(teardown: bool, test: T) -> ()
    where
        T: FnOnce(&str) -> () + std::panic::UnwindSafe,
    {
        let db_name = Self::get_test_db_name();

        Self::setup(&db_name);
        let result = std::panic::catch_unwind(|| test(&db_name));
        if (teardown) {
            Self::teardown(&db_name);
        }

        assert!(result.is_ok())
    }

    pub fn run_test<T>(test: T) -> ()
    where
        T: FnOnce(&str) -> () + std::panic::UnwindSafe,
    {
        let db_name = Self::get_test_db_name();

        Self::setup(&db_name);
        let result = std::panic::catch_unwind(|| test(&db_name));
        Self::teardown(&db_name);

        assert!(result.is_ok())
    }

    fn get_test_db_name() -> String {
        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;
        let test_db_name = "fin_unit_test";
        format!("{}_{}", &test_db_name, *counter)
    }

    fn get_test_db_uri(db_name: &str) -> String {
        format!("{}/{}", CLUSTER_URI, db_name)
    }

    fn setup(db_name: &str) {
        // create database
        let db_conn = Connection::connect(CLUSTER_URI, TlsMode::None)
            .expect("unable to create db conn");
        db_conn
            .execute(&format!("CREATE DATABASE {name};", name = db_name), &[])
            .expect("unable to create db");

        // apply schema and add fake data
        let c_str = format!("{}/{}", CLUSTER_URI, db_name);
        let conn = Connection::connect(
            Self::get_test_db_uri(db_name).as_str(),
            TlsMode::None,
        )
        .unwrap();
        let init =
            fs::read_to_string("migrations/2018-10-07-022941_init/up.sql")
                .expect("file not found");
        let fake_data =
            fs::read_to_string("migrations/2018-10-07-232226_fake_data/up.sql")
                .expect("file not found");

        conn.batch_execute(&init).unwrap();
        conn.batch_execute(&fake_data).unwrap();
    }

    fn teardown(db_name: &str) {
        let db_conn = Connection::connect(CLUSTER_URI, TlsMode::None)
            .expect("unable to delete db conn");

        db_conn
            .execute(&format!("DROP database {};", db_name), &[])
            .expect("unable to delete db");
    }
}
