use crate::{
	communication::{benefit, cost, peers::KnownPeers, notification::Share},
    LOG_TARGET,
};

use std::{sync::Arc, time::Duration, fmt::Display};

use sp_application_crypto::{AppPublic, RuntimeAppPublic};

use codec::{Encode, Decode, DecodeWithMemTracking};
use scale_info::TypeInfo;

use sc_network::{NetworkPeers, ReputationChange};

const REBROADCAST_AFTER: Duration = Duration::from_secs(5);

#[derive(Debug, PartialEq)]
pub(super) enum Action<H> {
	// repropagate under given topic, to the given peers, applying cost/benefit to originator.
	Keep(H, ReputationChange),
	// discard, applying cost/benefit to originator.
	Discard(ReputationChange),
	// ignore, no cost/benefit applied to originator.
	DiscardNoReport,
}

/// An outcome of examining a message.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Consider {
	/// Accept the message.
	Accept,
	/// Message cannot be evaluated. Reject.
	CannotEvaluate,
}

/// DKG Share Message.
///
/// A Share message is an elliptic curve point shared in our DKG scheme with a BLS signature attached
#[derive(Clone, Debug, Decode, DecodeWithMemTracking, Encode, PartialEq, TypeInfo)]
pub struct ShareMessage<Share, Id, Signature> {
	/// Commit to information extracted from a finalized block
	pub share: Share,
    /// Node authority id
	pub id: Id,
	/// Node signature
	pub signature: Signature,
}

/// DKG gossip message type that gets encoded and sent on the network.
#[derive(Debug, Encode, Decode)]
pub(crate) enum GossipMessage<AuthorityId: AuthorityIdBound> {
	/// DKG message with commitment and single signature.
	Share(ShareMessage<Share<Vec<u8>>, AuthorityId, <AuthorityId as RuntimeAppPublic>::Signature>),
    // TODO: Add another message?
    Temp,
}

pub trait AuthorityIdBound:
    Ord
	+ AppPublic
    + Display
    + RuntimeAppPublic
{

}

// TEMP replace with BLS-381..
// TODO: Goes in a primitive folder for DKG
/// DKG cryptographic types for ECDSA crypto
///
/// This module basically introduces four crypto types:
/// - `ecdsa_crypto::Pair`
/// - `ecdsa_crypto::Public`
/// - `ecdsa_crypto::Signature`
/// - `ecdsa_crypto::AuthorityId`
///
/// Your code should use the above types as concrete types for all crypto related
/// functionality.
pub mod ecdsa_crypto {
	use super::{AuthorityIdBound, RuntimeAppPublic};
	use sp_application_crypto::{app_crypto, ecdsa};

    // TODO: Put this in Keytypes module in crypto similar to babe beefy etc..
    // sp_application_crypto::key_types
    /// Key type for DKG module.
	pub const DKG: sp_core::crypto::KeyTypeId = sp_core::crypto::KeyTypeId(*b"dkgg");

	app_crypto!(ecdsa, DKG);

	/// Identity of a DKG authority using ECDSA as its crypto.
	pub type AuthorityId = Public;

	/// Signature for a DKG authority using ECDSA as its crypto.
	pub type AuthoritySignature = Signature;

	impl AuthorityIdBound for AuthorityId {

	}
}