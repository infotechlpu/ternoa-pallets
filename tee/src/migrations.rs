use super::*;

pub mod v2 {
	use super::*;
	use frame_support::{
		traits::OnRuntimeUpgrade, CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound,
	};
	use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
	use scale_info::TypeInfo;
	use sp_std::fmt::Debug;

	#[derive(
		Encode,
		Decode,
		CloneNoBound,
		PartialEqNoBound,
		Eq,
		RuntimeDebugNoBound,
		TypeInfo,
		MaxEncodedLen,
	)]
	#[scale_info(skip_type_params(ClusterSize))]
	#[codec(mel_bound(AccountId: MaxEncodedLen))]
	pub struct OldClusterData<AccountId, ClusterSize>
	where
		AccountId: Clone + PartialEq + Debug,
		ClusterSize: Get<u32>,
	{
		pub enclaves: BoundedVec<AccountId, ClusterSize>,
	}

	pub struct MigrationV2<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrationV2<T> {
		// #[cfg(feature = "try-runtime")]
		// fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		// 	log::info!("Pre-upgrade inside MigrationV2");
		// 	Ok(Vec::new())
		// }

		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let mut read = 0u64;
			let mut write = 0u64;

			let current_active_era = Staking::<T>::active_era().map(|e| e.index).unwrap();
			read += 1;

			ClusterData::<T>::translate(
				|_id, old: OldClusterData<T::AccountId, T::ClusterSize>| {
					let mut new_enclaves: BoundedVec<(T::AccountId, SlotId), T::ClusterSize> =
						BoundedVec::default();
					
					let mut slot_id_counter = 0;

					for account_id in old.enclaves.into_iter() {
						let slot_id: SlotId = slot_id_counter;
						slot_id_counter += 1;

						let push_result = new_enclaves.try_push((account_id.clone(), slot_id));
						match push_result {
							Ok(_) => {
								read += 1;
								write += 1;
							},
							Err(_) => {
								// Handle the error case if the `BoundedVec` is already full
								break // Stop adding elements if the desired size is reached
							},
						}
						OperatorAssignedEra::<T>::insert(account_id.clone(), current_active_era);
						write += 1;

						let new_staking_ledger =
						TeeStakingLedger::new(account_id.clone(), BalanceOf::<T>::default(), false, Default::default());
						StakingLedger::<T>::insert(account_id.clone(), new_staking_ledger);
						write += 1;

					}
					let new_cluster_data = Cluster::new(new_enclaves, ClusterType::Public);
					read += 1;
					write += 1;

					Some(new_cluster_data)
				},
			);

			T::DbWeight::get().reads_writes(read, write)
		}

		// #[cfg(feature = "try-runtime")]
		// fn post_upgrade(_: Vec<u8>) -> Result<(), &'static str> {
		// 	log::info!("Post-upgrade inside MigrationV2");
		// 	Ok(())
		// }
	}
}