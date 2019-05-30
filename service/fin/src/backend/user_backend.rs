use crate::data;
use crate::errors::*;

pub trait UserBackend {
    fn get_user(&self, email: &String) -> ResultFin<data::UserData>;

    fn get_user_with_pass(
        &self,
        email: &String,
    ) -> ResultFin<data::UserDataWithPass>;

    fn does_user_exist(&self, email: &String) -> ResultFin<bool>;

    fn create_user(
        &self,
        email: &String,
        password: &String,
    ) -> ResultFin<data::UserData>;
}

impl UserBackend {
    pub fn get_logger_context(logger: slog::Logger) -> slog::Logger {
        logger.new(o!("mod" => "user_backend"))
    }
}

pub struct DefaultUserBackend<T: data::FinDb> {
    db: T,
}

impl<T: data::FinDb> DefaultUserBackend<T> {
    pub fn new(db: T) -> DefaultUserBackend<T> {
        DefaultUserBackend { db: db }
    }
}

impl<T: data::FinDb> UserBackend for DefaultUserBackend<T> {
    fn get_user(&self, email: &String) -> ResultFin<data::UserData> {
        self.db.get_user(email)
    }

    fn get_user_with_pass(
        &self,
        email: &String,
    ) -> ResultFin<data::UserDataWithPass> {
        self.db.get_user_with_pass(email)
    }

    fn does_user_exist(&self, email: &String) -> ResultFin<bool> {
        self.db.does_user_exist(email)
    }

    fn create_user(
        &self,
        email: &String,
        password: &String,
    ) -> ResultFin<data::UserData> {
        self.db.create_user(email, password)
    }
}
