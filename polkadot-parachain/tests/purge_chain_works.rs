// Copyright 2020-2021 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use assert_cmd::cargo::cargo_bin;
use nix::sys::signal::SIGINT;
use std::process::Command;
use tempfile::tempdir;

mod common;

#[tokio::test]
#[cfg(unix)]
#[ignore]
async fn purge_chain_works() {
	// Check that both databases are deleted

	let base_dir = tempdir().expect("could not create a temp dir");

	let args = &["--", "--dev"];

	common::run_node_for_a_while(base_dir.path(), args, SIGINT).await;

	assert!(base_dir.path().join("chains/local_testnet/db/full").exists());
	assert!(base_dir.path().join("polkadot/chains/dev/db/full").exists());

	let status = Command::new(cargo_bin("polkadot-parachain"))
		.args(&["purge-chain", "-d"])
		.arg(base_dir.path())
		.arg("-y")
		.status()
		.unwrap();
	assert!(status.success());

	// Make sure that the `parachain_local_testnet` chain folder exists, but the `db` is deleted.
	assert!(base_dir.path().join("chains/local_testnet").exists());
	assert!(!base_dir.path().join("chains/local_testnet/db/full").exists());
	// assert!(base_path.path().join("polkadot/chains/dev").exists());
	// assert!(!base_path.path().join("polkadot/chains/dev/db").exists());
}
