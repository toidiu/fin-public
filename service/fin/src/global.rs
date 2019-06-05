use slog::*;
use std::fs::{self, OpenOptions};

lazy_static! {
    pub static ref CONFIG: fin_config::FinConfig =
        fin_config::FinConfig::new().expect("unable to parse configs");


    static ref log_dir: &'static str = "./logs";
    static ref _n: () = fs::create_dir_all(&*log_dir).expect("unable to create dir for logs");
    static ref log_path: String = format!("{}/server.log", &*log_dir);
    static ref file: std::fs::File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        // .truncate(true)
        .open(&*log_path)
        .expect("failed to open log file");

    static ref drain: slog::Fuse<slog_async::Async> = slog_async::Async::new(
        slog_term::FullFormat::new(slog_term::PlainDecorator::new(&*file)).build().fuse(),
    )
    .build()
    .fuse();

    pub static ref ROOT: slog::Logger =
        slog::Logger::root(&*drain, o!("crate" => "fin", "version" => env!("CARGO_PKG_VERSION")));
}
