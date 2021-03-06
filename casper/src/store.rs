// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate Shasper.

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

//! Common store traits

use num_traits::{One, Zero};
use rstd::prelude::*;
use rstd::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

/// Casper attestation. The source should always be canon.
pub trait Attestation: PartialEq + Eq {
	/// Type of validator Id.
	type ValidatorId: PartialEq + Eq + Clone + Copy;
	/// Type of validator Id collection.
	type ValidatorIdIterator: IntoIterator<Item=Self::ValidatorId>;
	/// Type of epoch.
	type Epoch: PartialEq + Eq + PartialOrd + Ord + Clone + Copy + Add<Output=Self::Epoch> + AddAssign + Sub<Output=Self::Epoch> + SubAssign + One + Zero;

	/// Get validator Ids of this attestation.
	fn validator_ids(&self) -> Self::ValidatorIdIterator;
	/// Whether this attestation's source is on canon chain.
	fn is_source_canon(&self) -> bool;
	/// Whether this attestation's target is on canon chain.
	fn is_target_canon(&self) -> bool;
	/// Get the source epoch of this attestation.
	fn source_epoch(&self) -> Self::Epoch;
	/// Get the target epoch of this attestation.
	fn target_epoch(&self) -> Self::Epoch;

	/// Whether this attestation's source and target is on canon chain.
	fn is_casper_canon(&self) -> bool {
		self.is_source_canon() && self.is_target_canon()
	}
}

/// Store that holds validator active and balance information.
pub trait ValidatorStore {
	/// Type of validator Id.
	type ValidatorId: PartialEq + Eq + Clone + Copy;
	/// Type of validator Id collection.
	type ValidatorIdIterator: IntoIterator<Item=Self::ValidatorId>;
	/// Type of balance.
	type Balance: PartialEq + Eq + PartialOrd + Ord + Clone + Copy + Add<Output=Self::Balance> + AddAssign + Sub<Output=Self::Balance> + SubAssign + Mul<Output=Self::Balance> + Div<Output=Self::Balance> + From<u8>;
	/// Type of epoch.
	type Epoch: PartialEq + Eq + PartialOrd + Ord + Clone + Copy + Add<Output=Self::Epoch> + AddAssign + Sub<Output=Self::Epoch> + SubAssign + One + Zero;

	/// Get total balance of given validator Ids.
	fn total_balance(&self, validators: &[Self::ValidatorId]) -> Self::Balance;
	/// Get all active validators at given epoch.
	fn active_validators(&self, epoch: Self::Epoch) -> Self::ValidatorIdIterator;
}

/// Store that holds pending attestations.
pub trait PendingAttestationsStore {
	/// Type of attestation.
	type Attestation: Attestation;
	/// Type of attestation collection.
	type AttestationIterator: IntoIterator<Item=Self::Attestation>;

	/// Get the current list of attestations.
	fn attestations(&self) -> Self::AttestationIterator;
	/// Retain specific attestations and remove the rest.
	fn retain<F: FnMut(&Self::Attestation) -> bool>(&mut self, f: F);
}

/// Store that holds general block information.
pub trait BlockStore {
	/// Type of epoch.
	type Epoch: PartialEq + Eq + PartialOrd + Ord + Clone + Copy + Add<Output=Self::Epoch> + AddAssign + Sub<Output=Self::Epoch> + SubAssign + One + Zero;

	/// Get the current epoch.
	fn epoch(&self) -> Self::Epoch;
	/// Get the next epoch.
	fn next_epoch(&self) -> Self::Epoch {
		self.epoch() + One::one()
	}
	/// Get the previous epoch.
	fn previous_epoch(&self) -> Self::Epoch {
		if self.epoch() == Zero::zero() {
			Zero::zero()
		} else {
			self.epoch() - One::one()
		}
	}
}

/// Epoch of a pending attestation store.
pub type PendingAttestationsStoreEpoch<S> = <<S as PendingAttestationsStore>::Attestation as Attestation>::Epoch;
/// Validator Id of a pending attestation store.
pub type PendingAttestationsStoreValidatorId<S> = <<S as PendingAttestationsStore>::Attestation as Attestation>::ValidatorId;
/// Balance of a validator store.
pub type ValidatorStoreBalance<S> = <S as ValidatorStore>::Balance;
/// Epoch of a validator store.
pub type ValidatorStoreEpoch<S> = <S as ValidatorStore>::Epoch;
/// Validator id of a validator store.
pub type ValidatorStoreValidatorId<S> = <S as ValidatorStore>::ValidatorId;

/// Attesting canon target balance at epoch.
pub fn canon_target_attesting_balance<S>(store: &S, epoch: PendingAttestationsStoreEpoch<S>) -> ValidatorStoreBalance<S> where
	S: PendingAttestationsStore,
	S: ValidatorStore<
		ValidatorId=PendingAttestationsStoreValidatorId<S>,
		Epoch=PendingAttestationsStoreEpoch<S>
	>,
{
	let mut validators = Vec::new();
	for attestation in store.attestations() {
		if attestation.is_casper_canon() && attestation.target_epoch() == epoch {
			for validator_id in attestation.validator_ids() {
				validators.push(validator_id.clone());
			}
		}
	}
	store.total_balance(&validators)
}

/// Attesting canon source balance at epoch.
pub fn canon_source_attesting_balance<S>(store: &S, epoch: PendingAttestationsStoreEpoch<S>) -> ValidatorStoreBalance<S> where
	S: PendingAttestationsStore,
	S: ValidatorStore<
		ValidatorId=PendingAttestationsStoreValidatorId<S>,
		Epoch=PendingAttestationsStoreEpoch<S>
	>,
{
	let mut validators = Vec::new();
	for attestation in store.attestations() {
		if attestation.is_casper_canon() && attestation.source_epoch() == epoch {
			for validator_id in attestation.validator_ids() {
				validators.push(validator_id.clone());
			}
		}
	}
	store.total_balance(&validators)
}

/// Total balance at epoch.
pub fn active_total_balance<S>(store: &S, epoch: ValidatorStoreEpoch<S>) -> ValidatorStoreBalance<S> where
	S: ValidatorStore
{
	let validators = store.active_validators(epoch).into_iter().collect::<Vec<_>>();
	store.total_balance(&validators)
}
