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

//! Autogenerated weights for pallet_core_fellowship
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ugdxctqu-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_core_fellowship
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/core-fellowship/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_core_fellowship.
pub trait WeightInfo {
	fn set_params() -> Weight;
	fn bump_offboard() -> Weight;
	fn bump_demote() -> Weight;
	fn set_active() -> Weight;
	fn induct() -> Weight;
	fn promote() -> Weight;
	fn offboard() -> Weight;
	fn import() -> Weight;
	fn approve() -> Weight;
	fn submit_evidence() -> Weight;
}

/// Weights for pallet_core_fellowship using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: CoreFellowship Params (r:0 w:1)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	fn set_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_313_000 picoseconds.
		Weight::from_parts(10_545_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Params (r:1 w:0)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:1 w:0)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn bump_offboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16854`
		//  Estimated: `19894`
		// Minimum execution time: 62_546_000 picoseconds.
		Weight::from_parts(65_542_000, 19894)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Params (r:1 w:0)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:1 w:0)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn bump_demote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16964`
		//  Estimated: `19894`
		// Minimum execution time: 66_311_000 picoseconds.
		Weight::from_parts(68_834_000, 19894)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn set_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `355`
		//  Estimated: `3514`
		// Minimum execution time: 18_974_000 picoseconds.
		Weight::from_parts(19_814_000, 3514)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn induct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `3514`
		// Minimum execution time: 29_381_000 picoseconds.
		Weight::from_parts(30_239_000, 3514)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Params (r:1 w:0)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn promote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16832`
		//  Estimated: `19894`
		// Minimum execution time: 61_737_000 picoseconds.
		Weight::from_parts(63_002_000, 19894)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:0 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn offboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `326`
		//  Estimated: `3514`
		// Minimum execution time: 18_823_000 picoseconds.
		Weight::from_parts(19_509_000, 3514)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	fn import() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3514`
		// Minimum execution time: 18_276_000 picoseconds.
		Weight::from_parts(18_920_000, 3514)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16810`
		//  Estimated: `19894`
		// Minimum execution time: 43_536_000 picoseconds.
		Weight::from_parts(44_665_000, 19894)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:0)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn submit_evidence() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `19894`
		// Minimum execution time: 30_159_000 picoseconds.
		Weight::from_parts(31_025_000, 19894)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: CoreFellowship Params (r:0 w:1)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	fn set_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_313_000 picoseconds.
		Weight::from_parts(10_545_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Params (r:1 w:0)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:1 w:0)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn bump_offboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16854`
		//  Estimated: `19894`
		// Minimum execution time: 62_546_000 picoseconds.
		Weight::from_parts(65_542_000, 19894)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Params (r:1 w:0)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:1 w:0)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn bump_demote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16964`
		//  Estimated: `19894`
		// Minimum execution time: 66_311_000 picoseconds.
		Weight::from_parts(68_834_000, 19894)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn set_active() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `355`
		//  Estimated: `3514`
		// Minimum execution time: 18_974_000 picoseconds.
		Weight::from_parts(19_814_000, 3514)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn induct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `3514`
		// Minimum execution time: 29_381_000 picoseconds.
		Weight::from_parts(30_239_000, 3514)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:1)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Params (r:1 w:0)
	/// Proof: CoreFellowship Params (max_values: Some(1), max_size: Some(364), added: 859, mode: MaxEncodedLen)
	/// Storage: RankedCollective MemberCount (r:1 w:1)
	/// Proof: RankedCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	/// Storage: RankedCollective IndexToId (r:0 w:1)
	/// Proof: RankedCollective IndexToId (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// Proof: RankedCollective IdToIndex (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	fn promote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16832`
		//  Estimated: `19894`
		// Minimum execution time: 61_737_000 picoseconds.
		Weight::from_parts(63_002_000, 19894)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:0 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn offboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `326`
		//  Estimated: `3514`
		// Minimum execution time: 18_823_000 picoseconds.
		Weight::from_parts(19_509_000, 3514)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	fn import() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3514`
		// Minimum execution time: 18_276_000 picoseconds.
		Weight::from_parts(18_920_000, 3514)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: CoreFellowship Member (r:1 w:1)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16810`
		//  Estimated: `19894`
		// Minimum execution time: 43_536_000 picoseconds.
		Weight::from_parts(44_665_000, 19894)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: CoreFellowship Member (r:1 w:0)
	/// Proof: CoreFellowship Member (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: CoreFellowship MemberEvidence (r:1 w:1)
	/// Proof: CoreFellowship MemberEvidence (max_values: None, max_size: Some(16429), added: 18904, mode: MaxEncodedLen)
	fn submit_evidence() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `19894`
		// Minimum execution time: 30_159_000 picoseconds.
		Weight::from_parts(31_025_000, 19894)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
