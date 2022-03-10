use tracing::{span, trace, Level};
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        // works with global filter
        // .with(fmt::Layer::default().and_then(EnvFilter::new("target1[span1]")))
        // bug with per-layer filter
        .with(fmt::Layer::default().with_filter(EnvFilter::new("target1[span1]")))
        .init();
    log_trace_for_target_and_field("val1");
}

fn log_trace_for_target_and_field(field_val: &str) {
    trace!(target: "target1", "should be ignored (correct target, but no field)");
    {
        let s = span!(target: "target1", Level::INFO, "span1", field1 = field_val);
        let _g = s.enter();
        trace!(target: "target1", "should be logged");
    }
    trace!(target: "target2", "should be ignored (wrong target, no field)");
    {
        let s = span!(target: "target2", Level::INFO, "span2", field2 = field_val);
        let _g = s.enter();
        trace!(target: "target2", "should be ignored (correct field, but wrong target)");
    }
}
