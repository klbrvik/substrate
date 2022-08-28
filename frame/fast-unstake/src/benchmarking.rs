// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
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

//! Benchmarking for pallet-example-basic.

#![cfg(feature = "runtime-benchmarks")]

use crate::{Pallet as FastUnstake, *};
use frame_benchmarking::{benchmarks, whitelist_account};
use frame_support::{
	assert_ok,
	traits::{Currency, EnsureOrigin, Get, Hooks},
};
use frame_system::RawOrigin;
use pallet_nomination_pools::{Pallet as Pools, PoolId};
use pallet_staking::Pallet as Staking;
use sp_runtime::traits::{StaticLookup, Zero};
use sp_staking::EraIndex;
use sp_std::prelude::*;

const USER_SEED: u32 = 0;
const DEFAULT_BACKER_PER_VALIDATOR: u32 = 128;

type CurrencyOf<T> = <T as pallet_staking::Config>::Currency;

fn l<T: Config>(
	who: T::AccountId,
) -> <<T as frame_system::Config>::Lookup as StaticLookup>::Source {
	T::Lookup::unlookup(who)
}

fn create_unexposed_nominator<T: Config>() -> T::AccountId {
	let account = frame_benchmarking::account::<T::AccountId>("nominator_42", 0, USER_SEED);
	let stake = CurrencyOf::<T>::minimum_balance() * 100u32.into();
	CurrencyOf::<T>::make_free_balance_be(&account, stake * 10u32.into());

	let account_lookup = l::<T>(account.clone());
	// bond and nominate ourselves, this will guarantee that we are not backing anyone.
	Staking::<T>::bond(
		RawOrigin::Signed(account.clone()).into(),
		account_lookup.clone(),
		stake,
		pallet_staking::RewardDestination::Controller,
	)
	.unwrap();
	Staking::<T>::nominate(RawOrigin::Signed(account.clone()).into(), vec![account_lookup])
		.unwrap();

	account
}

fn setup_pool<T: Config>() -> PoolId {
	let depositor = frame_benchmarking::account::<T::AccountId>("depositor_42", 0, USER_SEED);
	let depositor_lookup = l::<T>(depositor.clone());

	let stake = Pools::<T>::depositor_min_bond();
	CurrencyOf::<T>::make_free_balance_be(&depositor, stake * 10u32.into());

	Pools::<T>::create(
		RawOrigin::Signed(depositor.clone()).into(),
		stake,
		depositor_lookup.clone(),
		depositor_lookup.clone(),
		depositor_lookup,
	)
	.unwrap();

	pallet_nomination_pools::LastPoolId::<T>::get()
}

fn setup_staking<T: Config>(v: u32, until: EraIndex) {
	let ed = CurrencyOf::<T>::minimum_balance();

	log!(debug, "registering {} validators and {} eras.", v, until);

	// our validators don't actually need to registered in staking -- just generate `v` random
	// accounts.
	let validators = (0..v)
		.map(|x| frame_benchmarking::account::<T::AccountId>("validator", x, USER_SEED))
		.collect::<Vec<_>>();

	for era in 0..=until {
		let others = (0..DEFAULT_BACKER_PER_VALIDATOR)
			.map(|s| {
				let who = frame_benchmarking::account::<T::AccountId>("nominator", era, s);
				let value = ed;
				pallet_staking::IndividualExposure { who, value }
			})
			.collect::<Vec<_>>();
		let exposure =
			pallet_staking::Exposure { total: Default::default(), own: Default::default(), others };
		validators.iter().for_each(|v| {
			Staking::<T>::add_era_stakers(era, v.clone(), exposure.clone());
		});
	}
}

fn on_idle_full_block<T: Config>() {
	let remaining_weight = <T as frame_system::Config>::BlockWeights::get().max_block;
	FastUnstake::<T>::on_idle(Zero::zero(), remaining_weight);
}

benchmarks! {
	// on_idle, we we don't check anyone, but fully unbond and move them to another pool.
	on_idle_unstake {
		let who = create_unexposed_nominator::<T>();
		let pool_id = setup_pool::<T>();
		assert_ok!(FastUnstake::<T>::register_fast_unstake(
			RawOrigin::Signed(who.clone()).into(),
			Some(pool_id)
		));
		ErasToCheckPerBlock::<T>::put(1);

		// run on_idle once. This will check era 0.
		assert_eq!(Head::<T>::get(), None);
		on_idle_full_block::<T>();
		assert_eq!(
			Head::<T>::get(),
			Some(UnstakeRequest { stash: who.clone(), checked: vec![0], maybe_pool_id: Some(1) })
		);
	}
	: {
		on_idle_full_block::<T>();
	}
	verify {
		// TODO: make sure who os not staked anymore, and has indeed joined a pool.
	}

	// on_idle, when we check some number of eras,
	on_idle_check {
		// number of validators
		let v in 1 .. 1000;
		// number of eras to check.
		let u in 1 .. (<T as pallet_staking::Config>::BondingDuration::get() / 4);

		ErasToCheckPerBlock::<T>::put(u);
		pallet_staking::CurrentEra::<T>::put(u);

		// setup staking with v validators and u eras of data (0..=u)
		setup_staking::<T>(v, u);
		let who = create_unexposed_nominator::<T>();
		assert_ok!(FastUnstake::<T>::register_fast_unstake(
			RawOrigin::Signed(who.clone()).into(),
			None,
		));

		// no one is queued thus far.
		assert_eq!(Head::<T>::get(), None);
	}
	: {
		on_idle_full_block::<T>();
	}
	verify {
		let checked = (1..=u).rev().collect::<Vec<EraIndex>>();
		assert_eq!(
			Head::<T>::get(),
			Some(UnstakeRequest { stash: who.clone(), checked, maybe_pool_id: None })
		);
	}

	// same as above, but we do the entire check and realize that we had to slash our nominator now.
	on_idle_check_slash {}
	: {}
	verify {}

	register_fast_unstake {
		let who = create_unexposed_nominator::<T>();
		whitelist_account!(who);
	}
	:_(RawOrigin::Signed(who.clone()), None)
	verify {}

	deregister {
		let who = create_unexposed_nominator::<T>();
		assert_ok!(FastUnstake::<T>::register_fast_unstake(
			RawOrigin::Signed(who.clone()).into(),
			None
		));
		whitelist_account!(who);
	}
	:_(RawOrigin::Signed(who.clone()))
	verify {}

	control {
		let origin = <T as Config>::ControlOrigin::successful_origin();
	}
	: _<T::Origin>(origin, 128)
	verify {}

	// impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test)
}
