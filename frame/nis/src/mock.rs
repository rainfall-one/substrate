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

//! Test environment for NIS pallet.

use crate::{self as pallet_nis, Perquintill, WithMaximumOf};

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{
		fungible::Inspect, ConstU16, ConstU32, ConstU64, Everything, OnFinalize, OnInitialize,
		StorageMapShim,
	},
	weights::Weight,
	PalletId,
};
use pallet_balances::{Instance1, Instance2};
use scale_info::TypeInfo;
use sp_core::{ConstU128, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances::<Instance1>,
		Nis: pallet_nis,
		NisBalances: pallet_balances::<Instance2> = 45,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

type NisCounterpartInstance = pallet_balances::Instance2;
impl pallet_balances::Config<NisCounterpartInstance> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<50>; // One KTC cent
	type AccountStore = StorageMapShim<
		pallet_balances::Account<Test, NisCounterpartInstance>,
		Self::AccountId,
		pallet_balances::AccountData<u128>,
	>;
	type MaxLocks = ConstU32<4>;
	type MaxReserves = ConstU32<4>;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type HoldIdentifier = ();
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

impl pallet_balances::Config<Instance1> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<10>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<1>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type HoldIdentifier = HoldIdentifier;
	type MaxHolds = ConstU32<1>;
}

#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, MaxEncodedLen, Debug, TypeInfo,
)]
pub enum HoldIdentifier {
	Nis,
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

parameter_types! {
	pub static Target: Perquintill = Perquintill::zero();
	pub MinReceipt: Perquintill = Perquintill::from_rational(1u64, 10_000_000u64);
	pub const ThawThrottle: (Perquintill, u64) = (Perquintill::from_percent(25), 5);
	pub storage NisTarget: Perquintill = Perquintill::zero();
	pub const NisPalletId: PalletId = PalletId(*b"py/nis  ");
	pub static MaxIntakeWeight: Weight = Weight::from_parts(2_000_000_000_000, 0);
	pub const HoldReason: HoldIdentifier = HoldIdentifier::Nis;
}

impl pallet_nis::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type FundOrigin = frame_system::EnsureSigned<Self::AccountId>;
	type Counterpart = NisBalances;
	type CounterpartAmount = WithMaximumOf<ConstU128<21_000_000_000_000_000_000u128>>;
	type Deficit = (); // Mint
	type IgnoredIssuance = ();
	type Target = NisTarget;
	type PalletId = NisPalletId;
	type QueueCount = ConstU32<500>;
	type MaxQueueLen = ConstU32<1000>;
	type FifoQueueLen = ConstU32<250>;
	type BasePeriod = ConstU64<3>;
	type MinBid = ConstU128<2>;
	type MinReceipt = MinReceipt;
	type IntakePeriod = ConstU64<2>;
	type MaxIntakeWeight = MaxIntakeWeight;
	type ThawThrottle = ThawThrottle;
	type HoldReason = HoldReason;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test, Instance1> {
		balances: vec![(1, 100), (2, 100), (3, 100), (4, 100)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup, but without any balances.
#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext_empty() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		Nis::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		Nis::on_initialize(System::block_number());
	}
}
