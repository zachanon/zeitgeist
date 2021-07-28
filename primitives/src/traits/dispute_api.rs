use crate::{market::MarketDispute, types::Market};
use frame_support::dispatch::DispatchResult;

/// Dispute Api
pub trait DisputeApi {
    type AccountId;
    type Balance;
    type BlockNumber;
    type MarketId;
    type Origin;

    /// Disputes a reported outcome.
    fn on_dispute(
        disputes: &[MarketDispute<Self::AccountId, Self::BlockNumber>],
        market_id: Self::MarketId,
    ) -> DispatchResult;

    /// Manages markets resolutions moving all reported markets to resolved.
    fn on_resolution<F>(now: Self::BlockNumber, cb: F) -> DispatchResult
    where
        F: FnMut(&Self::MarketId, &Market<Self::AccountId, Self::BlockNumber>) -> DispatchResult;
}
