pub use pallet::*;


#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    use ark_ec::pairing::Pairing;
    use ark_ec::{CurveGroup, PrimeGroup};
    use ark_poly::GeneralEvaluationDomain;
    use ark_std::test_rng;
    use adkg_vrf::bls::threshold::ThresholdVk;


    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProofVerified { who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        VerificationFailed,
        DeserializationFailed,
    }

    #[pallet::storage]
    #[pallet::getter(fn aggregated_transcript)]
    pub type AggregatedTranscript<T: Config> = StorageValue<_, Vec<u8>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000_000)]
        pub fn add_transcript(
            origin: OriginFor<T>,
            transcript_bytes: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let new_transcript = Transcript::<ark_bls12_381::Bls12_381>::deserialize_compressed(&transcript_bytes[..])
                .map_err(|_| Error::<T>::DeserializationFailed)?;

            // Get existing aggregated transcript if it exists
            let aggregated = if let Some(existing_bytes) = AggregatedTranscript::<T>::get() {
                // Deserialize existing
                let existing = Transcript::<ark_bls12_381::Bls12_381>::deserialize_compressed(&existing_bytes[..])
                    .map_err(|_| Error::<T>::DeserializationFailed)?;

                // Merge with new transcript using merge_with
                existing.merge_with(&vec![new_transcript])
            } else {
                // First transcript, just use it as is
                new_transcript
            };

            // Serialize and store the aggregated transcript
            let mut aggregated_bytes = Vec::new();
            aggregated.serialize_compressed(&mut aggregated_bytes)
                .map_err(|_| Error::<T>::SerializationFailed)?;
            AggregatedTranscript::<T>::put(aggregated_bytes);

            ensure!(is_valid, Error::<T>::VerificationFailed);

            Self::deposit_event(Event::ProofVerified { who });
            Ok(())
        }

        #[pallet::call_index(0)]
        #[pallet::weight(10_000_000)]
        pub fn compute_vk(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
              // Get the aggregated transcript
            let transcript_bytes = AggregatedTranscript::<T>::get()
                .ok_or(Error::<T>::NoAggregatedTranscript)?;

            let agg_transcript = Transcript::<ark_bls12_381::Bls12_381>::deserialize_compressed(&transcript_bytes[..])
                .map_err(|_| Error::<T>::DeserializationFailed)?;

            let threshold_vk = adkg_vrf::ThresholdVk::from_share(&agg_transcript.payload);

            Ok(())
        }
    }
}

