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

//! Autogenerated weights for pallet_lottery
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
// --pallet=pallet_lottery
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/lottery/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_lottery.
pub trait WeightInfo {
	fn buy_ticket() -> Weight;
	fn set_calls(n: u32, ) -> Weight;
	fn start_lottery() -> Weight;
	fn stop_repeat() -> Weight;
	fn on_initialize_end() -> Weight;
	fn on_initialize_repeat() -> Weight;
}

/// Weights for pallet_lottery using the Substrate pivot and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Lottery Lottery (r:1 w:0)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Lottery CallIndices (r:1 w:0)
	/// Proof: Lottery CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// Storage: Lottery TicketsCount (r:1 w:1)
	/// Proof: Lottery TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Lottery Participants (r:1 w:1)
	/// Proof: Lottery Participants (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Lottery LotteryIndex (r:1 w:0)
	/// Proof: Lottery LotteryIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Lottery Tickets (r:0 w:1)
	/// Proof: Lottery Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn buy_ticket() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `3593`
		// Minimum execution time: 61_502_000 picoseconds.
		Weight::from_parts(62_578_000, 3593)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Lottery CallIndices (r:0 w:1)
	/// Proof: Lottery CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn set_calls(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_282_000 picoseconds.
		Weight::from_parts(9_271_031, 0)
			// Standard Error: 3_756
			.saturating_add(Weight::from_parts(349_990, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Lottery LotteryIndex (r:1 w:1)
	/// Proof: Lottery LotteryIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn start_lottery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `3593`
		// Minimum execution time: 38_975_000 picoseconds.
		Weight::from_parts(39_552_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	fn stop_repeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `1514`
		// Minimum execution time: 8_243_000 picoseconds.
		Weight::from_parts(8_359_000, 1514)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Lottery TicketsCount (r:1 w:1)
	/// Proof: Lottery TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Lottery Tickets (r:1 w:0)
	/// Proof: Lottery Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn on_initialize_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `524`
		//  Estimated: `6196`
		// Minimum execution time: 76_062_000 picoseconds.
		Weight::from_parts(76_547_000, 6196)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Lottery TicketsCount (r:1 w:1)
	/// Proof: Lottery TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Lottery Tickets (r:1 w:0)
	/// Proof: Lottery Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Lottery LotteryIndex (r:1 w:1)
	/// Proof: Lottery LotteryIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_repeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `524`
		//  Estimated: `6196`
		// Minimum execution time: 78_089_000 picoseconds.
		Weight::from_parts(78_632_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Lottery Lottery (r:1 w:0)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Lottery CallIndices (r:1 w:0)
	/// Proof: Lottery CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// Storage: Lottery TicketsCount (r:1 w:1)
	/// Proof: Lottery TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Lottery Participants (r:1 w:1)
	/// Proof: Lottery Participants (max_values: None, max_size: Some(65), added: 2540, mode: MaxEncodedLen)
	/// Storage: Lottery LotteryIndex (r:1 w:0)
	/// Proof: Lottery LotteryIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Lottery Tickets (r:0 w:1)
	/// Proof: Lottery Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn buy_ticket() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `3593`
		// Minimum execution time: 61_502_000 picoseconds.
		Weight::from_parts(62_578_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Lottery CallIndices (r:0 w:1)
	/// Proof: Lottery CallIndices (max_values: Some(1), max_size: Some(21), added: 516, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn set_calls(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_282_000 picoseconds.
		Weight::from_parts(9_271_031, 0)
			// Standard Error: 3_756
			.saturating_add(Weight::from_parts(349_990, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: Lottery LotteryIndex (r:1 w:1)
	/// Proof: Lottery LotteryIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn start_lottery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `3593`
		// Minimum execution time: 38_975_000 picoseconds.
		Weight::from_parts(39_552_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	fn stop_repeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `1514`
		// Minimum execution time: 8_243_000 picoseconds.
		Weight::from_parts(8_359_000, 1514)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Lottery TicketsCount (r:1 w:1)
	/// Proof: Lottery TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Lottery Tickets (r:1 w:0)
	/// Proof: Lottery Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn on_initialize_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `524`
		//  Estimated: `6196`
		// Minimum execution time: 76_062_000 picoseconds.
		Weight::from_parts(76_547_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	/// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: MaxEncodedLen)
	/// Storage: Lottery Lottery (r:1 w:1)
	/// Proof: Lottery Lottery (max_values: Some(1), max_size: Some(29), added: 524, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Lottery TicketsCount (r:1 w:1)
	/// Proof: Lottery TicketsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Lottery Tickets (r:1 w:0)
	/// Proof: Lottery Tickets (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Lottery LotteryIndex (r:1 w:1)
	/// Proof: Lottery LotteryIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_repeat() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `524`
		//  Estimated: `6196`
		// Minimum execution time: 78_089_000 picoseconds.
		Weight::from_parts(78_632_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}