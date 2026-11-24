pub fn log_init(_log_name: &str) {
    init_console();
}

pub fn log_init0(_log_name: &str, _yaml: Option<&str>) {
    init_console();
}

fn init_console() {
    let _ = log4rs::init_str(
        r#"
appenders:
  console:
    kind: console
    encoder:
      pattern: "{d} [{l}] {m}{n}"
root:
  level: info
  appenders: [console]
"#,
    );
}
