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

//! Autogenerated weights for pallet_message_queue
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-w5zkt3rr-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_message_queue
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/message-queue/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_message_queue.
pub trait WeightInfo {
	fn ready_ring_knit() -> Weight;
	fn ready_ring_unknit() -> Weight;
	fn service_queue_base() -> Weight;
	fn service_page_base_completion() -> Weight;
	fn service_page_base_no_completion() -> Weight;
	fn service_page_item() -> Weight;
	fn bump_service_head() -> Weight;
	fn reap_page() -> Weight;
	fn execute_overweight_page_removed() -> Weight;
	fn execute_overweight_page_updated() -> Weight;
}

/// Weights for pallet_message_queue using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `6038`
		// Minimum execution time: 12_602_000 picoseconds.
		Weight::from_parts(13_195_000, 6038)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `6038`
		// Minimum execution time: 11_746_000 picoseconds.
		Weight::from_parts(12_316_000, 6038)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3514`
		// Minimum execution time: 4_870_000 picoseconds.
		Weight::from_parts(5_051_000, 3514)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `69049`
		// Minimum execution time: 6_178_000 picoseconds.
		Weight::from_parts(6_505_000, 69049)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `69049`
		// Minimum execution time: 6_670_000 picoseconds.
		Weight::from_parts(6_964_000, 69049)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 66_789_000 picoseconds.
		Weight::from_parts(67_168_000, 0)
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3514`
		// Minimum execution time: 7_137_000 picoseconds.
		Weight::from_parts(7_527_000, 3514)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65710`
		//  Estimated: `69049`
		// Minimum execution time: 60_660_000 picoseconds.
		Weight::from_parts(64_415_000, 69049)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65710`
		//  Estimated: `69049`
		// Minimum execution time: 78_147_000 picoseconds.
		Weight::from_parts(79_892_000, 69049)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65710`
		//  Estimated: `69049`
		// Minimum execution time: 93_178_000 picoseconds.
		Weight::from_parts(95_311_000, 69049)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `6038`
		// Minimum execution time: 12_602_000 picoseconds.
		Weight::from_parts(13_195_000, 6038)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `6038`
		// Minimum execution time: 11_746_000 picoseconds.
		Weight::from_parts(12_316_000, 6038)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3514`
		// Minimum execution time: 4_870_000 picoseconds.
		Weight::from_parts(5_051_000, 3514)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `69049`
		// Minimum execution time: 6_178_000 picoseconds.
		Weight::from_parts(6_505_000, 69049)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `69049`
		// Minimum execution time: 6_670_000 picoseconds.
		Weight::from_parts(6_964_000, 69049)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 66_789_000 picoseconds.
		Weight::from_parts(67_168_000, 0)
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3514`
		// Minimum execution time: 7_137_000 picoseconds.
		Weight::from_parts(7_527_000, 3514)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65710`
		//  Estimated: `69049`
		// Minimum execution time: 60_660_000 picoseconds.
		Weight::from_parts(64_415_000, 69049)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65710`
		//  Estimated: `69049`
		// Minimum execution time: 78_147_000 picoseconds.
		Weight::from_parts(79_892_000, 69049)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65584), added: 68059, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65710`
		//  Estimated: `69049`
		// Minimum execution time: 93_178_000 picoseconds.
		Weight::from_parts(95_311_000, 69049)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
