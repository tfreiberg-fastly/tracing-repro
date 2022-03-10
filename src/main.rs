use std::str::FromStr;

use tracing::{info, span, trace, Level};
use tracing_subscriber::{
    filter::{EnvFilter, Filtered, Targets},
    fmt,
    prelude::*,
    reload::{self, Handle},
    util::SubscriberInitExt,
    Registry,
};

fn main() {
    // let (filter, handle) = reload::Layer::new(
    //     fmt::Layer::default().with_filter(EnvFilter::from_str("target1[span1]").unwrap()),
    // );
    tracing_subscriber::registry()
        .with(fmt::Layer::default().with_filter(EnvFilter::new("target1[span1]")))
        .init();
    log_trace_for_target_and_field("val1");
    // info!(target: "target", "logged");
    // trace!(target: "target", "ignored (level too low)");
    // info!(target: "other-target", "ignored (wrong target)");

    // // reload_filter(&handle, "target");
    // reload_filter(&handle, "target[span{field=val1}]");

    // log_trace_for_target_and_field("val1");

    // reload_filter(&handle, "target[span{field=val1}]=trace");
    // // reload_filter(&handle, "target=trace");

    // log_trace_for_target_and_field("val1");
}

fn reload_filter(
    handle: &Handle<Filtered<fmt::Layer<Registry>, EnvFilter, Registry>, Registry>,
    filter: &str,
) {
    let env_filter = EnvFilter::from_str(filter).unwrap();
    println!("Reloading with {env_filter}");
    // println!("dbg: env_filter:?}");
    handle.modify(|f| *f.filter_mut() = env_filter).unwrap();
}

fn log_trace_for_target_and_field(field_val: &str) {
    trace!(target: "target1", "ignored (correct target, but no field)");
    {
        let s = span!(target: "target1", Level::INFO, "span1", field1 = field_val);
        let _g = s.enter();
        trace!(target: "target1", "logged");
    }
    trace!(target: "target2", "ignored (wrong target, no field)");
    {
        let s = span!(target: "target2", Level::INFO, "span2", field2 = field_val);
        let _g = s.enter();
        trace!(target: "target2", "ignored (correct field, but wrong target)");
    }
}
