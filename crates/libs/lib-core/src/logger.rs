use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;

use crate::config::Env;

pub fn init(env: Env) -> WorkerGuard {
	let file_logger = tracing_appender::rolling::daily("logs", "align.log");
	let console_logger = std::io::stdout();

	let max_level = match env {
		Env::Development => LevelFilter::DEBUG,
		Env::Production => LevelFilter::DEBUG,
	};

	let (non_blocking, guard) = match env {
		Env::Development => tracing_appender::non_blocking(console_logger),
		Env::Production => tracing_appender::non_blocking(file_logger),
	};

	tracing_subscriber::fmt()
		.with_max_level(max_level)
		.with_writer(non_blocking)
		.init();

	guard
}
