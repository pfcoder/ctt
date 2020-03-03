// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! Democracy pallet benchmarking.

use super::*;

use frame_system::{RawOrigin, self};
use sp_io::hashing::blake2_256;
use frame_benchmarking::{benchmarks, account};
use sp_core::hash::H256;
use sp_runtime::traits::{Bounded, Dispatchable, StaticLookup};
use frame_support::traits::{Currency, Get};

use crate::Module as Democracy;

const SEED: u32 = 0;
const MAX_PROPOSALS: u32 = 100;
const MAX_REFERENDUMS: u32 = 100;
const MAX_USERS: u32 = 100;

fn add_proposals<T: Trait>(number: u32) -> Result<(), &'static str> {
	#[cfg(feature = "std")]
	println!("add_proposal {}", number);

	for p in 0 .. number {
		let other: T::AccountId = account("other", p, SEED);
		let _ = T::Currency::make_free_balance_be(&other, BalanceOf::<T>::max_value());
		let value = T::MinimumDeposit::get();

		let proposal_hash: T::Hash = Default::default();

		Democracy::<T>::propose(RawOrigin::Signed(other).into(), proposal_hash, value.into())?;
	}
	Ok(())
}

fn add_referendums<T: Trait>(number: u32) -> Result<(), &'static str> {
	#[cfg(feature = "std")]
	println!("add_referendum {}", number);

	for p in 0 .. number {
		
		add_proposals::<T>(1);
		
		let vote_threshold = VoteThreshold::SimpleMajority;
		Democracy::<T>::inject_referendum(
			0.into(),
			Default::default(),
			vote_threshold,
			0.into(),
		);
	}
	Ok(())
}

benchmarks! {
	_ {
		let p in 1 .. MAX_PROPOSALS => add_proposals::<T>(p)?;
		let r in 1 .. MAX_REFERENDUMS => add_referendums::<T>(r)?;
		let u in 1 .. MAX_USERS => ();
	}

	propose {
		// The execution time doesn't seems to change depending on the number of proposals.
		let p in ...;

		let caller: T::AccountId = account("caller", 0, SEED);
		let proposal_hash: T::Hash = Default::default();
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		let value = T::MinimumDeposit::get();

	}: _(RawOrigin::Signed(caller), proposal_hash, value.into())

	second {
		// The execution time doesn't seems to change depending on the number of proposals.
		let p in ...;

		let caller: T::AccountId = account("caller", 0, SEED);
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		// The index of the proposal p is (p - 1);
	}: _(RawOrigin::Signed(caller), (p - 1).into())

	vote {
		// The execution time doesn't seems to change depending on inputs.
		let u in ...;

		let caller: T::AccountId = account("caller", u, SEED);
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		// Add a proposal.
		add_proposals::<T>(1);

		// Inject referendum.
		let proposal_hash: T::Hash = Default::default();
		let vote_threshold = VoteThreshold::SimpleMajority;
		Democracy::<T>::inject_referendum(
			0.into(),
			proposal_hash,
			vote_threshold,
			0.into(),
		);

		// Vote.
		let v = Vote {
			aye: true,
			conviction: Conviction::Locked1x,
		};

	}: _(RawOrigin::Signed(caller), 0u32.into(), v)

	proxy_vote {
		// The execution time doesn't seems to change depending on inputs.
		let u in ...;

		let caller: T::AccountId = account("caller", u, SEED);
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		let proxy: T::AccountId = account("proxy", u + MAX_USERS, SEED);
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());

		let r = Democracy::<T>::open_proxy(RawOrigin::Signed(proxy.clone()).into(), caller.clone());
		#[cfg(feature = "std")]
		println!("result of open proxy {:?}", r);

		let r = Democracy::<T>::activate_proxy(RawOrigin::Signed(caller).into(), proxy.clone());
		#[cfg(feature = "std")]
		println!("result of activate proxy {:?}", r);
					
		add_proposals::<T>(1);

		let proposal_hash: T::Hash = Default::default();
		let vote_threshold = VoteThreshold::SimpleMajority;
		Democracy::<T>::inject_referendum(
			0.into(),
			proposal_hash,
			vote_threshold,
			0.into(),
		);

		let v = Vote {
			aye: true,
			conviction: Conviction::Locked1x,
		};

	}: _(RawOrigin::Signed(proxy), 0u32.into(), v)
}