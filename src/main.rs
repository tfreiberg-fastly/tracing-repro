use tracing::{error, trace};
use tracing_subscriber::{
    filter::EnvFilter, fmt, prelude::*, reload, util::SubscriberInitExt, Layer, Registry,
};

mod tuple_layer;

// The issue is that I want to see the following logs:
// TRACE tracing_bug::a: trace in a
// ERROR tracing_bug::a: error in a
// ERROR tracing_bug::b: error in b
//
// but when using `and_then` / `Layered`, nothing is logged at all
fn main() {
    let (layer_a, _handle_a) = layer_a();
    let (layer_b, _fmt_handle, _filter_handle_b) = layer_b();

    let max_a = layer_a.max_level_hint();
    let max_b = layer_b.max_level_hint();

    // this does not work
    // let combined = layer_a.and_then(layer_b);

    // this does
    let combined = crate::tuple_layer::TupleLayer(layer_a, layer_b);

    // and it's caused by combined.max_level_hint() being OFF
    println!(
        "a: {max_a:?} + b: {max_b:?} = {:?}",
        combined.max_level_hint()
    );

    Registry::default().with(combined).init();

    a::log();
    b::log();
}

type FmtLayer = fmt::Layer<Registry>;

type Handle<T> = reload::Handle<T, Registry>;

fn layer_a() -> (impl Layer<Registry>, Handle<EnvFilter>) {
    let fmt_layer = fmt::Layer::default().with_ansi(false);
    let filter = EnvFilter::new("tracing_bug::a=trace,error");
    let (filter, handle) = reload::Layer::new(filter);
    let layer = fmt_layer.with_filter(filter);
    (layer, handle)
}

fn layer_b() -> (
    impl Layer<Registry>,
    Handle<Option<FmtLayer>>,
    Handle<EnvFilter>,
) {
    // in the real code, this is a layer that accepts a unix stream and streams logs to it as long as it's attached
    let fmt_layer: Option<fmt::Layer<Registry>> = None;
    let (fmt_layer, fmt_handle) = reload::Layer::new(fmt_layer);
    let filter = EnvFilter::default();
    let (filter, filter_handle) = reload::Layer::new(filter);
    let layer = fmt_layer.with_filter(filter);
    (layer, fmt_handle, filter_handle)
}

mod a {
    use super::*;

    pub fn log() {
        trace!("trace in a");
        error!("error in a");
    }
}

mod b {
    use super::*;

    pub fn log() {
        trace!("trace in b");
        error!("error in b");
    }
}
