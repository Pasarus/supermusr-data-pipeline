pub(crate) use common::metrics::{
    failures::{FailureKind, FailureLabels},
    messages_received::{MessageKind, MessagesReceivedLabels},
};
use kagiyama::{AlwaysReady, Watcher};
use lazy_static::lazy_static;
use prometheus_client::metrics::{counter::Counter, family::Family};

pub(crate) fn register(watcher: &Watcher<AlwaysReady>) {
    let mut registry = watcher.metrics_registry();

    let registry = registry.sub_registry_with_prefix("eventstohistogram");

    registry.register(
        "messages_processed",
        "Messages succesfully processed and published",
        Box::new(MESSAGES_PROCESSED.clone()),
    );

    registry.register("failures", "Failures by type", Box::new(FAILURES.clone()));

    registry.register(
        "messages_received",
        "Messages received by type from incomming Kafka topic",
        Box::new(MESSAGES_RECEIVED.clone()),
    );
}

lazy_static! {
    pub(crate) static ref MESSAGES_PROCESSED: Counter = Counter::default();
    pub(crate) static ref FAILURES: Family::<FailureLabels, Counter> =
        Family::<FailureLabels, Counter>::default();
    pub(crate) static ref MESSAGES_RECEIVED: Family::<MessagesReceivedLabels, Counter> =
        Family::<MessagesReceivedLabels, Counter>::default();
}
