// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate Shasper.

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

use primitives::ValidatorId;
use runtime::GenesisConfig;
use crypto::bls;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate ChainSpec type.
pub type ChainSpec = service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	/// Weekly short-lived public testnet.
	PublicShortLived
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"development",
				|| {
					let alice_sec = bls::Secret::from_bytes(b"Alice").unwrap();
					let alice = bls::Pair::from_secret(alice_sec);
					let alice_id = ValidatorId::from_public(alice.public.clone());

					testnet_genesis(
						vec![
							alice_id
						]
					)
				},
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| {
					let alice_sec = bls::Secret::from_bytes(b"Alice").unwrap();
					let bob_sec = bls::Secret::from_bytes(b"Bob").unwrap();

					let alice = bls::Pair::from_secret(alice_sec);
					let bob = bls::Pair::from_secret(bob_sec);

					let alice_id = ValidatorId::from_public(alice.public.clone());
					let bob_id = ValidatorId::from_public(bob.public.clone());

					testnet_genesis(
						vec![
							alice_id,
							bob_id,
						]
					)
				},
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::PublicShortLived => ChainSpec::from_genesis(
				"Public Short-lived Testnet",
				"short-lived",
				|| {
					use std::str::FromStr;

					let alice_id = ValidatorId::from_str("8e9bed908372adcb328d242bc0f03fa527a232d2c7e81daa8b350a7593cf5c89a62795909ef18ed09cf8e24123076ce9").expect("Preloaded key is valid; qed");
					let bob_id = ValidatorId::from_str("a5908e909329db6e5ba9083aae10b6b1dd9341ed6f70f3395a59b944b059737cbce745664fb31f087c6d5f74756619a3").expect("Preloaded key is valid; qed");

					testnet_genesis(
						vec![
							alice_id,
							bob_id,
						]
					)
				},
				vec![],
				None,
				None,
				None,
				None
			),
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"local" => Some(Alternative::LocalTestnet),
			"short-lived" => Some(Alternative::PublicShortLived),
			_ => None,
		}
	}
}

fn testnet_genesis(initial_authorities: Vec<ValidatorId>) -> GenesisConfig {
	GenesisConfig {
		code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/shasper_runtime.compact.wasm").to_vec(),
		authorities: initial_authorities.into_iter().map(|v| (v, 1000000)).collect(),
	}
}
