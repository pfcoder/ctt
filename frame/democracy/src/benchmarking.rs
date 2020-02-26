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

use frame_system::RawOrigin;
use sp_io::hashing::blake2_256;
use frame_benchmarking::{benchmarks, account};
use sp_core::H256;
use sp_runtime::traits::{Bounded, Dispatchable};
use pallet_balances::Module as Balances;
use crate::Module as Democracy;

const MAX_PROPOSALS = 100;

bechmarks! {
	_ {
		let p in 0 .. MAX_PROPOSALS => ();
	}

	propose {
		let p in ...;

		let minimum_deposit = T::MinimumDeposit::get();
		let caller = account("caller", 0, 0);
		Balances::set_ballance(RawOrigin::Root, caller, minimum_deposit);
		let proposal_hash = H256::random();

	}: _(RawOrigin::Signed(caller), proposal_hash, value)
}