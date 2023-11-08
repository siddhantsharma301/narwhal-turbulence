use crate::tests::common;
use config::{Parameters, KeyPair};
use consensus::Consensus;
use primary::Primary;
use store::Store;
use worker::Worker;
use tokio::sync::mpsc::channel;
use turmoil::{net, Builder};

/// The default channel capacity.
pub const CHANNEL_CAPACITY: usize = 1_000;

#[tokio::test]
async fn test() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    // Start simulation
    let mut sim = Builder::new().build();

    // Generate overall values
    let mut keys = common::keys();
    let committee = common::committee();
    let params = Parameters::default();

    sim.host("primary0", move || {
        let (primary0_pk, primary0_sk) = keys.pop().unwrap();
        let primary0_keypair = KeyPair {
            name: primary0_pk,
            secret: primary0_sk
        };
        let store0 = Store::new(".test-db-0").unwrap();

        let (tx0_output, rx0_output) = channel(CHANNEL_CAPACITY);
        let (tx0_new_certificates, rx0_new_certificates) = channel(CHANNEL_CAPACITY);
        let (tx0_feedback, rx0_feedback) = channel(CHANNEL_CAPACITY);

        Primary::spawn(
            primary0_keypair,
            committee,
            params,
            store0,
            tx0_new_certificates,
            rx0_feedback,
        );

        Consensus::spawn(
            committee,
            params.gc_depth,
            rx0_new_certificates,
            tx0_feedback,
            tx0_output
        );
    });
}

