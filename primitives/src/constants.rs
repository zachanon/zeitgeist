#![allow(
  // Constants parameters inside `parameter_types!` already check
  // arithmetic operations at compile time
    clippy::integer_arithmetic
)]

pub mod ztg;

use crate::{
    asset::Asset,
    types::{AccountId, AccountIdTest, Balance, BlockNumber, CurrencyId},
};
use frame_support::{parameter_types, PalletId};
use sp_runtime::{traits::AccountIdConversion, Permill};

// Definitions for time
pub const BLOCKS_PER_DAY: BlockNumber = BLOCKS_PER_HOUR * 24;
pub const MILLISECS_PER_BLOCK: u32 = 12000;
pub const BLOCKS_PER_MINUTE: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const BLOCKS_PER_HOUR: BlockNumber = BLOCKS_PER_MINUTE * 60;

// Definitions for currency
pub const BASE: u128 = 10_000_000_000;
pub const CENT: Balance = BASE / 100; // 100_000_000
pub const MILLI: Balance = CENT / 10; //  10_000_000
pub const MICRO: Balance = MILLI / 1000; // 10_000

pub const BALANCE_FRACTIONAL_DECIMAL_PLACES: u8 = {
    let mut base = BASE;
    let mut counter: u8 = 0;

    while base >= 10 {
        base /= 10;
        counter += 1;
    }

    counter
};

// Balance
parameter_types! {
    pub const ExistentialDeposit: u128 = CENT;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

// Court
parameter_types! {
    pub const CourtCaseDuration: u64 = BLOCKS_PER_DAY;
    pub const CourtPalletId: PalletId = PalletId(*b"zge/cout");
    pub const StakeWeight: u128 = 2 * BASE;
}

// General
parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

// Liquidity Mining parameters
parameter_types! {
    pub const LiquidityMiningPalletId: PalletId = PalletId(*b"zge/lymg");
}

// ORML
parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = Asset::Ztg;
    pub DustAccount: AccountId = PalletId(*b"orml/dst").into_account();
    pub DustAccountTest: AccountIdTest = PalletId(*b"orml/dst").into_account();
}

// Prediction Market parameters
parameter_types! {
    pub const AdvisoryBond: Balance = 25 * CENT;
    pub const DisputeBond: Balance = 5 * BASE;
    pub const DisputeFactor: Balance = 2 * BASE;
    pub const DisputePeriod: BlockNumber = BLOCKS_PER_DAY;
    pub const MaxCategories: u16 = 10;
    pub const MaxDisputes: u16 = 6;
    pub const MinCategories: u16 = 2;
    pub const OracleBond: Balance = 50 * CENT;
    pub const PmPalletId: PalletId = PalletId(*b"zge/pred");
    pub const ReportingPeriod: u32 = BLOCKS_PER_DAY as _;
    pub const ValidityBond: Balance = 50 * CENT;
}

// Simple disputes parameters
parameter_types! {
    pub const SimpleDisputesPalletId: PalletId = PalletId(*b"zge/sedp");
}

// Staking parameters
parameter_types! {
    pub const DefaultBlocksPerRound: u32 = 2 * BLOCKS_PER_MINUTE as u32;
}

// Swaps parameters
parameter_types! {
    pub const ExitFee: Balance = 0;
    pub const MaxAssets: u16 = MaxCategories::get() + 1;
    pub const MaxInRatio: Balance = BASE / 2;
    pub const MaxOutRatio: Balance = (BASE / 3) + 1;
    pub const MaxTotalWeight: Balance = 50 * BASE;
    pub const MaxWeight: Balance = 50 * BASE;
    pub const MinLiquidity: Balance = 100 * BASE;
    pub const MinWeight: Balance = BASE;
    pub const SwapsPalletId: PalletId = PalletId(*b"zge/swap");
}

// Time
parameter_types! {
    pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK as u64 / 2;
}

// Treasury
parameter_types! {
    pub const Burn: Permill = Permill::from_percent(50);
    pub const MaxApprovals: u32 = 100;
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 10 * BASE;
    pub const SpendPeriod: BlockNumber = BLOCKS_PER_DAY;
    pub const TreasuryPalletId: PalletId = PalletId(*b"zge/tsry");
}
