pub mod communication;

use std::{sync::Arc, marker::PhantomData};
use sc_network::{NotificationService, ProtocolName};

const LOG_TARGET: &str = "dkg";

/// DKG gadget network parameters.
pub struct DkgNetworkParams<N, S> {
	/// Network implementing gossip, requests and sync-oracle.
	pub network: Arc<N>,
	/// Syncing service implementing a sync oracle and an event stream for peers.
	pub sync: Arc<S>,
	/// Handle for receiving notification events.
	pub notification_service: Box<dyn NotificationService>,
	/// Chain specific DKG gossip protocol name. See
	/// [`communication::dkg_protocol_name::gossip_protocol_name`].
	pub gossip_protocol_name: ProtocolName,

	pub _phantom: PhantomData<(N, S)>,
}

