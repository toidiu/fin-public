use crate::errors::{FinError, ResultFin};
use crate::global::{CONFIG, ROOT};
use postgres_mapper;
use postgres_mapper::FromPostgresRow;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

lazy_static! {
    static ref CONNECTION: r2d2::Pool<PostgresConnectionManager> = {
        let manager = PostgresConnectionManager::new(
            CONFIG.database.url.to_string(),
            TlsMode::None,
        )
        .unwrap();
        r2d2::Pool::builder()
            .max_size(CONFIG.database.pool_size)
            .build(manager)
            .expect("Failed to create pool")
    };
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "server"));
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i64,
    pub email: String,
}

pub fn run(email: &str) {
    let conn = CONNECTION.get().unwrap();
    let db: TestDrop = TestDrop::new(conn, (*LOGGER).clone());

    println!("==========");
    println!("{:?}", db.sql_query(email).unwrap());
}

pub struct TestDrop {
    pub conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
    logger: slog::Logger,
}

impl TestDrop {
    pub fn new(
        conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
        logger: slog::Logger,
    ) -> Self {
        TestDrop {
            conn: conn,
            logger: logger.new(o!("mod" => "data")),
        }
    }

    pub fn sql_query(&self, email: &str) -> ResultFin<User> {
        // table users
        let stmt = &format!(
            "SELECT {} FROM {} WHERE email = $1",
            &User::sql_fields(),
            &User::sql_table(),
        );

        let rows = &self.conn.query(stmt, &[&email]).map_err(|err| {
            error!(self.logger, "{}: {}", line!(), err);
            lineError!(self.logger, err);
            FinError::DatabaseErr
        })?;

        let ret: ResultFin<User> = rows
            .iter()
            .next()
            .map(|row| {
                User::from_postgres_row(row).map_err(|err| {
                    error!(self.logger, "{}: {}", line!(), err);
                    FinError::DatabaseErr
                })
            })
            .ok_or_else(|| {
                lineError!(
                    self.logger,
                    format!("failed to get user for email: {}", &email)
                );
                FinError::DatabaseErr
            })?;

        ret
    }
}
