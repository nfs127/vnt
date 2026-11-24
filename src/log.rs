use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn log_init(_log_name: &str) {
    init_console();
}

pub fn log_init0(_log_name: &str, _yaml: Option<&str>) {
    init_console();
}

fn init_console() {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {h({l})} {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))
        .expect("log4rs config");

    let _ = log4rs::init_config(config);
}
