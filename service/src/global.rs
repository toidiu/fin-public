use slog::*;
use std::fs::{self, OpenOptions};

lazy_static! {
    pub static ref CONFIG: fin_config::FinConfig =
        fin_config::FinConfig::new().expect("unable to parse configs");


    pub static ref ROOT: slog::Logger = {
        let log_dir: &'static str = "./logs";
        let n: () = fs::create_dir_all(log_dir).expect("unable to create dir for logs");
        let log_path: String = format!("{}/server.log", log_dir);
        let file: std::fs::File = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            // .truncate(true)
            .open(&*log_path)
            .expect("failed to open log file");

        // terminal output in development
        #[cfg(debug_assertions)]
        let formatter = slog_term::FullFormat::new(
                slog_term::PlainDecorator::new(file)
            ).build();
        // json formatting in production
        #[cfg(not(debug_assertions))]
        let formatter = slog_bunyan::default(file);

        let fuse = slog_async::Async::new(formatter.fuse()).build().fuse();
        slog::Logger::root(fuse, o!("crate" => "fin", "version" => env!("CARGO_PKG_VERSION")))


    };

}
