// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! White labeled polkadot omni-node.
//!
//! For documentation, see [`polkadot_omni_node_lib`].

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

use polkadot_omni_node_lib::{
	chain_spec::DiskChainSpecLoader, run, runtime::DefaultRuntimeResolver, CliConfig as CliConfigT,
	RunConfig, NODE_VERSION,
};

struct CliConfig;

impl CliConfigT for CliConfig {
	fn impl_version() -> String {
		let commit_hash = env!("SUBSTRATE_CLI_COMMIT_HASH");
		format!("{}-{commit_hash}", NODE_VERSION)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/polkadot-sdk/issues/new".into()
	}

	fn copyright_start_year() -> u16 {
		2017
	}
}

fn main() -> color_eyre::eyre::Result<()> {
	color_eyre::install()?;

	let config = RunConfig::new(Box::new(DefaultRuntimeResolver), Box::new(DiskChainSpecLoader));
	Ok(run::<CliConfig>(config)?)
}
