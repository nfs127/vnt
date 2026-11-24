use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};

pub fn log_init(_log_name: &str) {
    init_console();
}

pub fn log_init0(_log_name: &str, _yaml: Option<&str>) {
    init_console();
}

fn init_console() {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .expect("log4rs config");

    let _ = log4rs::init_config(config);
}
