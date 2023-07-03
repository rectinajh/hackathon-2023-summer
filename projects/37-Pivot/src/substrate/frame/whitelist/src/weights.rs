// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_whitelist
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_whitelist
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/whitelist/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_whitelist.
pub trait WeightInfo {
	fn whitelist_call() -> Weight;
	fn remove_whitelisted_call() -> Weight;
	fn dispatch_whitelisted_call(n: u32, ) -> Weight;
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight;
}

/// Weights for pallet_whitelist using the Substrate pivot and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn whitelist_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `3556`
		// Minimum execution time: 21_370_000 picoseconds.
		Weight::from_parts(21_834_000, 3556)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn remove_whitelisted_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3556`
		// Minimum execution time: 19_222_000 picoseconds.
		Weight::from_parts(19_582_000, 3556)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 4194294]`.
	fn dispatch_whitelisted_call(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `422 + n * (1 ±0)`
		//  Estimated: `3886 + n * (1 ±0)`
		// Minimum execution time: 31_417_000 picoseconds.
		Weight::from_parts(31_620_000, 3886)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_145, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3556`
		// Minimum execution time: 23_092_000 picoseconds.
		Weight::from_parts(24_043_432, 3556)
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_227, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn whitelist_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `3556`
		// Minimum execution time: 21_370_000 picoseconds.
		Weight::from_parts(21_834_000, 3556)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn remove_whitelisted_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3556`
		// Minimum execution time: 19_222_000 picoseconds.
		Weight::from_parts(19_582_000, 3556)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 4194294]`.
	fn dispatch_whitelisted_call(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `422 + n * (1 ±0)`
		//  Estimated: `3886 + n * (1 ±0)`
		// Minimum execution time: 31_417_000 picoseconds.
		Weight::from_parts(31_620_000, 3886)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_145, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `346`
		//  Estimated: `3556`
		// Minimum execution time: 23_092_000 picoseconds.
		Weight::from_parts(24_043_432, 3556)
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_227, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}