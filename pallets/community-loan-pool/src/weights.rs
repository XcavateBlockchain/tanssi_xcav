
//! Autogenerated weights for `pallet_community_loan_pool`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-DFFNONK6`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_community_loan_pool
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/community-loan-pool/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions needed for pallet_community_loan_pool.
pub trait WeightInfo {
	fn propose() -> Weight;
	fn add_committee_member() -> Weight;
	fn set_milestones() -> Weight;
	fn vote_on_proposal() -> Weight;
	fn withdraw() -> Weight;
	fn repay() -> Weight;
	fn propose_milestone() -> Weight;
	fn vote_on_milestone_proposal() -> Weight;
	fn propose_deletion() -> Weight;
	fn vote_on_deletion_proposal() -> Weight;
}

/// Weight functions for `pallet_community_loan_pool`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::ReservedLoanAmount` (r:1 w:1)
	/// Proof: `CommunityLoanPool::ReservedLoanAmount` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::ProposalCount` (r:1 w:1)
	/// Proof: `CommunityLoanPool::ProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::RoundsExpiring` (r:1 w:1)
	/// Proof: `CommunityLoanPool::RoundsExpiring` (`max_values`: None, `max_size`: Some(4022), added: 6497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingVotes` (r:0 w:1)
	/// Proof: `CommunityLoanPool::OngoingVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Proposals` (r:0 w:1)
	/// Proof: `CommunityLoanPool::Proposals` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `448`
		//  Estimated: `3201489`
		// Minimum execution time: 66_054_000 picoseconds.
		Weight::from_parts(69_453_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `CommunityLoanPool::VotingCommittee` (r:1 w:1)
	/// Proof: `CommunityLoanPool::VotingCommittee` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `MaxEncodedLen`)
	fn add_committee_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `33487`
		// Minimum execution time: 19_454_000 picoseconds.
		Weight::from_parts(22_397_000, 0)
			.saturating_add(Weight::from_parts(0, 33487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::VotingCommittee` (r:1 w:0)
	/// Proof: `CommunityLoanPool::VotingCommittee` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Proposals` (r:1 w:1)
	/// Proof: `CommunityLoanPool::Proposals` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::OngoingVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::UserVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::UserVotes` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn set_milestones() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `616`
		//  Estimated: `3201489`
		// Minimum execution time: 36_293_000 picoseconds.
		Weight::from_parts(37_886_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::VotingCommittee` (r:1 w:0)
	/// Proof: `CommunityLoanPool::VotingCommittee` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::OngoingVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::UserVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::UserVotes` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Proposals` (r:1 w:0)
	/// Proof: `CommunityLoanPool::Proposals` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	fn vote_on_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `785`
		//  Estimated: `3201489`
		// Minimum execution time: 34_560_000 picoseconds.
		Weight::from_parts(36_767_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Loans` (r:1 w:1)
	/// Proof: `CommunityLoanPool::Loans` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::ReservedLoanAmount` (r:1 w:1)
	/// Proof: `CommunityLoanPool::ReservedLoanAmount` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `974`
		//  Estimated: `3201489`
		// Minimum execution time: 81_022_000 picoseconds.
		Weight::from_parts(83_718_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Loans` (r:1 w:1)
	/// Proof: `CommunityLoanPool::Loans` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::TotalLoanAmount` (r:1 w:1)
	/// Proof: `CommunityLoanPool::TotalLoanAmount` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn repay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `923`
		//  Estimated: `3201489`
		// Minimum execution time: 73_694_000 picoseconds.
		Weight::from_parts(78_134_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Loans` (r:1 w:0)
	/// Proof: `CommunityLoanPool::Loans` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::MilestoneProposalCount` (r:1 w:1)
	/// Proof: `CommunityLoanPool::MilestoneProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::MilestoneRoundsExpiring` (r:1 w:1)
	/// Proof: `CommunityLoanPool::MilestoneRoundsExpiring` (`max_values`: None, `max_size`: Some(4022), added: 6497, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::MilestoneInfo` (r:0 w:1)
	/// Proof: `CommunityLoanPool::MilestoneInfo` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingMilestoneVotes` (r:0 w:1)
	/// Proof: `CommunityLoanPool::OngoingMilestoneVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::MilestoneProposals` (r:0 w:1)
	/// Proof: `CommunityLoanPool::MilestoneProposals` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn propose_milestone() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `788`
		//  Estimated: `3201489`
		// Minimum execution time: 50_419_000 picoseconds.
		Weight::from_parts(56_122_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::VotingCommittee` (r:1 w:0)
	/// Proof: `CommunityLoanPool::VotingCommittee` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingMilestoneVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::OngoingMilestoneVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::UserMilestoneVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::UserMilestoneVotes` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn vote_on_milestone_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `725`
		//  Estimated: `3201489`
		// Minimum execution time: 30_895_000 picoseconds.
		Weight::from_parts(34_811_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::Loans` (r:1 w:1)
	/// Proof: `CommunityLoanPool::Loans` (`max_values`: None, `max_size`: Some(168), added: 2643, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::DeletionProposalCount` (r:1 w:1)
	/// Proof: `CommunityLoanPool::DeletionProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::DeletionRoundsExpiring` (r:1 w:1)
	/// Proof: `CommunityLoanPool::DeletionRoundsExpiring` (`max_values`: None, `max_size`: Some(4022), added: 6497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::DeletionProposals` (r:0 w:1)
	/// Proof: `CommunityLoanPool::DeletionProposals` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingDeletionVotes` (r:0 w:1)
	/// Proof: `CommunityLoanPool::OngoingDeletionVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn propose_deletion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `648`
		//  Estimated: `3201489`
		// Minimum execution time: 32_821_000 picoseconds.
		Weight::from_parts(38_203_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Whitelist::WhitelistedAccounts` (r:1 w:0)
	/// Proof: `Whitelist::WhitelistedAccounts` (`max_values`: Some(1), `max_size`: Some(3200004), added: 3200499, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::VotingCommittee` (r:1 w:0)
	/// Proof: `CommunityLoanPool::VotingCommittee` (`max_values`: Some(1), `max_size`: Some(32002), added: 32497, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::OngoingDeletionVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::OngoingDeletionVotes` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `CommunityLoanPool::UserDeletionVotes` (r:1 w:1)
	/// Proof: `CommunityLoanPool::UserDeletionVotes` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	fn vote_on_deletion_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `747`
		//  Estimated: `3201489`
		// Minimum execution time: 30_195_000 picoseconds.
		Weight::from_parts(34_124_000, 0)
			.saturating_add(Weight::from_parts(0, 3201489))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
