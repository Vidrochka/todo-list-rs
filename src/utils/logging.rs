use std::env;

use slog::*;
use slog_scope::GlobalLoggerGuard;

const TODO_SERVICE_FILE_LOG_LEVEL_ENV: &str = "TODO_SERVICE_FILE_LOG_LEVEL";
const TODO_SERVICE_CONSOLE_LOG_LEVEL_ENV: &str = "TODO_SERVICE_CONSOLE_LOG_LEVEL";
const TODO_SERVICE_LOG_PATH_ENV: &str = "TODO_SERVICE_LOG_PATH";

pub fn create_logger() -> (Logger, GlobalLoggerGuard) {
    let file_log_level = env::var(TODO_SERVICE_FILE_LOG_LEVEL_ENV)
        .expect(&*format!("Env {TODO_SERVICE_FILE_LOG_LEVEL_ENV} not found"))
        .parse::<Level>()
        .expect(&*format!("Env {TODO_SERVICE_FILE_LOG_LEVEL_ENV} must be valid slog::Level"));


    let console_log_level = env::var(TODO_SERVICE_CONSOLE_LOG_LEVEL_ENV)
        .expect(&*format!("Env {TODO_SERVICE_CONSOLE_LOG_LEVEL_ENV} not found"))
        .parse::<Level>()
        .expect(&*format!("Env {TODO_SERVICE_CONSOLE_LOG_LEVEL_ENV} must be valid slog::Level"));

    let decorator = slog_term::TermDecorator::new().build();
    let terminal_drain = slog_term::FullFormat::new(decorator).use_custom_header_print(print_msg_header).build().filter_level(file_log_level).fuse();
    let terminal_drain = slog_async::Async::new(terminal_drain).build().fuse();

    let log_path = env::var(TODO_SERVICE_LOG_PATH_ENV)
        .expect(&*format!("Env {TODO_SERVICE_LOG_PATH_ENV} not found"));

    let file = std::fs::File::create(log_path).expect("Couldn't open log file");

    let drain = slog_json::Json::new(file)
        .set_pretty(true)
        .add_key_value(o!(
            "ts" => FnValue(move |_ : &Record| {
                time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .ok()
            }),
        ))
        .add_key_value(o!(
            "level" => FnValue(move |rinfo : &Record| {
                rinfo.level().as_short_str()
            }),
        ))
        .add_key_value(o!(
            "msg" => PushFnValue(move |record : &Record, ser| {
                ser.emit(record.msg())
            }),
        ))
        .build()
        .filter_level(console_log_level)
        .fuse();
        
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_logger = Logger::root(
        slog::Duplicate::new(terminal_drain, drain).fuse(),
        o!("version" => env!("CARGO_PKG_VERSION")),
    );

    let scope_guard = slog_scope::set_global_logger(root_logger.clone());
    slog_stdlog::init().unwrap();

    (root_logger, scope_guard)
}

use std::io::Write;

pub fn print_msg_header(
    fn_timestamp: &dyn slog_term::ThreadSafeTimestampFn<Output = std::io::Result<()>>,
    mut rd: &mut dyn slog_term::RecordDecorator,
    record: &Record,
    use_file_location: bool,
) -> std::io::Result<bool> {
    rd.start_timestamp()?;
    fn_timestamp(&mut rd)?;

    rd.start_level()?;
    write!(rd, "{}", record.level().as_short_str())?;

    if use_file_location {
        rd.start_location()?;
        write!(
            rd,
            "[{}:{}:{}]",
            record.location().file,
            record.location().line,
            record.location().column
        )?;
    }

    rd.start_whitespace()?;
    write!(rd, " ")?;

    rd.start_msg()?;
    let mut count_rd = slog_term::CountingWriter::new(&mut rd);
    write!(count_rd, "{}", record.msg())?;
    Ok(count_rd.count() != 0)
}