use frame_support::dispatch::DispatchError;
use zeitgeist_primitives::{
    traits::DisputeApi,
    types::{Market, MarketDispute, ResolutionCounters},
};

/// SimpleDisputes - Pallet Api
pub trait SimpleDisputesPalletApi: DisputeApi {
    /// Performs the logic for resolving a market, including slashing and distributing
    /// funds.
    ///
    /// NOTE: This function does not perform any checks on the market that is being given.
    /// In the function calling this you should that the market is already in a reported or
    /// disputed state.
    fn internal_resolve<D>(
        dispute_bound: &D,
        disputes: &[MarketDispute<Self::AccountId, Self::BlockNumber>],
        market_id: &Self::MarketId,
        market: &Market<Self::AccountId, Self::BlockNumber>,
    ) -> Result<ResolutionCounters, DispatchError>
    where
        D: Fn(usize) -> Self::Balance;
}
