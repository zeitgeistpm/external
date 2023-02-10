// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_author_mapping
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// ./tmp/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_author_mapping.
pub trait WeightInfo {
    #[rustfmt::skip]
	fn add_association() -> Weight;
    #[rustfmt::skip]
	fn update_association() -> Weight;
    #[rustfmt::skip]
	fn clear_association() -> Weight;
    #[rustfmt::skip]
	fn remove_keys() -> Weight;
    #[rustfmt::skip]
	fn set_keys() -> Weight;
}

/// Weights for pallet_author_mapping using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
	#[rustfmt::skip]
    fn add_association() -> Weight {
		Weight::from_ref_time(50_753_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
	#[rustfmt::skip]
    fn update_association() -> Weight {
		Weight::from_ref_time(41_499_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
	#[rustfmt::skip]
    fn clear_association() -> Weight {
		Weight::from_ref_time(55_398_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping NimbusLookup (r:1 w:1)
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
    fn remove_keys() -> Weight {
		Weight::from_ref_time(60_757_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping NimbusLookup (r:1 w:1)
    // Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
	#[rustfmt::skip]
    fn set_keys() -> Weight {
		Weight::from_ref_time(45_388_000_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
	#[rustfmt::skip]
    fn add_association() -> Weight {
		Weight::from_ref_time(50_753_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
	#[rustfmt::skip]
    fn update_association() -> Weight {
		Weight::from_ref_time(41_499_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
	#[rustfmt::skip]
    fn clear_association() -> Weight {
		Weight::from_ref_time(55_398_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping NimbusLookup (r:1 w:1)
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
    fn remove_keys() -> Weight {
		Weight::from_ref_time(60_757_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
    // Storage: AuthorMapping NimbusLookup (r:1 w:1)
    // Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
	#[rustfmt::skip]
    fn set_keys() -> Weight {
		Weight::from_ref_time(45_388_000_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
