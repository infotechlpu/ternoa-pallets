// Copyright 2023 Capsule Corp (France) SAS.
// This file is part of Ternoa.

// Ternoa is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Ternoa is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Ternoa.  If not, see <http://www.gnu.org/licenses/>.

use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn set_session_extra_reward_payout() -> Weight;
}

/// Weight functions for `ternoa_nfts`.
pub struct TernoaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for TernoaWeight<T> {
	fn set_session_extra_reward_payout() -> Weight {
		Weight::from_ref_time(10_000_000 as u64)
	}
}
