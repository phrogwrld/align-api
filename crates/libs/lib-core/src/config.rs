#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum Env {
	Development,
	Production,
}

#[derive(clap::Parser, Debug)]
pub struct AppConfig {
	#[clap(long, env, value_enum)]
	pub env: Env,

	#[clap(long, env, default_value = "3000")]
	pub port: u16,

	#[clap(long, env)]
	pub cors_origin: String,

	#[clap(long)]
	pub migrate: bool,
}
