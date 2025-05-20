use sc_utils::notification::{NotificationSender, NotificationStream, TracingKeyStr};
use sp_runtime::{scale_info::TypeInfo, traits::Hash};

// TODO: Put this somewhere else
/// DKG Share
#[derive(Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct Share<T> {
	pub share: T,
}

/// The sending half of the notifications channel(s) used to send
/// notifications about DKG Shares.
pub type DkgShareSender = NotificationSender<Share<Vec<u8>>>;

/// The receiving half of a notifications channel used to receive
/// notifications about DKG shares.
pub type DkgShareReceiver =
	NotificationStream<Share<Vec<u8>>, DkgShareTracingKey>;

/// Provides tracing key for DKG share stream.
#[derive(Clone)]
pub struct DkgShareTracingKey;
impl TracingKeyStr for DkgShareTracingKey {
	const TRACING_KEY: &'static str = "mpsc_dkg_share_notification_stream";
}

