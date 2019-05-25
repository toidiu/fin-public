lazy_static! {
    pub static ref CONFIG: fin_config::FinConfig =
        fin_config::FinConfig::new().expect("unable to parse configs");
}
