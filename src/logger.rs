use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use termcolor::{StandardStream, ColorChoice, Color, ColorSpec, WriteColor};
use ::std::io::Write;

struct Logger;

fn write_log(record: &Record, to_print: (Color, &str)) {
    let (color, level) = to_print;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    write!(&mut stdout, "[{}] ", level).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
    writeln!(&mut stdout, "{}", record.args()).unwrap();
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let to_print = match record.level() {
                Level::Error => { (Color::Red, "error") }
                Level::Warn => { (Color::Yellow, "warn") }
                Level::Info => { (Color::Cyan, "info") }
                Level::Debug => { (Color::Magenta, "debug") }
                Level::Trace => { (Color::Blue, "trace") }
            };

            write_log(record, to_print);
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}
