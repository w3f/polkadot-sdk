pub mod communication;

use std::{sync::Arc, marker::PhantomData};
use rand::rngs::OsRng;
use rand::RngCore;
use sc_network::{NotificationService, ProtocolName};
use adkg_vrf::dkg::transcript::{DkgTranscript};
use adkg_vrf::dkg::{Ceremony, DkgResult};
use adkg_vrf::dkg::dealer;

use adkg_vrf::bls::threshold::ThresholdVk;
use adkg_vrf::bls::vanilla::BlsSigner;

use ark_ec::pairing::Pairing;
use ark_ec::{CurveGroup, PrimeGroup};
use ark_poly::GeneralEvaluationDomain;

#[cfg(feature = "bls-experimental")]
use sp_core::bls::bls381::*;

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

pub fn perform_dealing<C: Pairing>() -> Option<DkgTranscript<C>> {
	let mut os_rng = OsRng;

	// TODO: Get this from somewhere
	let num_validators = 42;
	let f = num_validators;

	let (n, t, k) = (3 * f + 1, 2 * f + 1, f + 1);
	// TODO: Add real BLS keys.. These will be the bls keys for each validator
    let signers: Vec<BlsSigner<C>> = (0..n)
            .map(|_| BlsSigner::new(C::G2::generator(), &mut os_rng))
            .collect();
    let signers_pks: Vec<_> = signers.iter()
            .map(|s| s.bls_pk_g2)
            .collect();
	let params = Ceremony::<C, GeneralEvaluationDomain<C::ScalarField>>::setup(t, &signers_pks);
	let transcript = params.deal(&mut os_rng);

	Some(transcript)
}

