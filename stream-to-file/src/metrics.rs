pub(crate) use common::metrics::{
    failures::{FailureKind, FailureLabels},
    messages_received::{MessageKind, MessagesReceivedLabels},
};
use kagiyama::{AlwaysReady, Watcher};
use lazy_static::lazy_static;
use prometheus_client::metrics::{counter::Counter, family::Family};

pub(crate) fn register(watcher: &mut Watcher<AlwaysReady>) {
    let mut registry = watcher.metrics_registry();
    let registry = registry.sub_registry_with_prefix("streamtofile");

    registry.register(
        "messages_received",
        "Messages received by type from incomming Kafka topic",
        Box::new(MESSAGES_RECEIVED.clone()),
    );

    registry.register("failures", "Failures by type", Box::new(FAILURES.clone()));
}

lazy_static! {
    pub(crate) static ref MESSAGES_RECEIVED: Family::<MessagesReceivedLabels, Counter> =
        Family::<MessagesReceivedLabels, Counter>::default();
    pub(crate) static ref FAILURES: Family::<FailureLabels, Counter> =
        Family::<FailureLabels, Counter>::default();
}
