use tracing::{Subscriber, Id};

struct JEDICTLogger;

impl Subscriber for JEDICTLogger {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &tracing_core::span::Attributes<'_>) -> tracing_core::span::Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &tracing_core::span::Id, _values: &tracing_core::span::Record<'_>) {
        todo!()
    }

    fn record_follows_from(&self, _span: &tracing_core::span::Id, _follows: &tracing_core::span::Id) {
    }

    fn event(&self, _event: &tracing::Event<'_>) {
        todo!()
    }

    fn enter(&self, _span: &tracing_core::span::Id) {
    }

    fn exit(&self, _span: &tracing_core::span::Id) {
    }
}
