//! CTT Node CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
	let version = sc_cli::VersionInfo {
		name: "CTT Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "ctt",
		author: "ctt team",
		description: "CTT Node",
		support_url: "support.anonymous.an",
		copyright_start_year: 2019,
	};

	command::run(version)
}
