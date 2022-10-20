use tracing::{metadata::LevelFilter, span, subscriber::Interest, Subscriber};
use tracing_subscriber::Layer;

pub struct TupleLayer<A, B>(pub A, pub B);

impl<A, B, S> Layer<S> for TupleLayer<A, B>
where
    A: Layer<S>,
    B: Layer<S>,
    S: Subscriber,
{
    fn on_register_dispatch(&self, collector: &tracing::Dispatch) {
        self.0.on_register_dispatch(collector);
        self.1.on_register_dispatch(collector);
    }

    fn on_layer(&mut self, subscriber: &mut S) {
        self.0.on_layer(subscriber);
        self.1.on_layer(subscriber);
    }

    fn register_callsite(
        &self,
        metadata: &'static tracing::Metadata<'static>,
    ) -> tracing::subscriber::Interest {
        let mut interest = Interest::never();
        for new_interest in [
            self.0.register_callsite(metadata),
            self.1.register_callsite(metadata),
        ] {
            if (interest.is_sometimes() && new_interest.is_always())
                || (interest.is_never() && !new_interest.is_never())
            {
                interest = new_interest;
            }
        }
        interest
    }

    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        let a_enabled = self.0.enabled(metadata, ctx.clone());
        let b_enabled = self.1.enabled(metadata, ctx);
        a_enabled || b_enabled
    }

    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.0.on_new_span(attrs, id, ctx.clone());
        self.1.on_new_span(attrs, id, ctx);
    }

    fn max_level_hint(&self) -> Option<tracing::metadata::LevelFilter> {
        // Default to `OFF` if there are no inner layers.
        let mut max_level = LevelFilter::OFF;
        for l in [self.0.max_level_hint(), self.1.max_level_hint()] {
            // NOTE(eliza): this is slightly subtle: if *any* layer
            // returns `None`, we have to return `None`, assuming there is
            // no max level hint, since that particular layer cannot
            // provide a hint.
            let hint = l?;
            max_level = core::cmp::max(hint, max_level);
        }
        Some(max_level)
    }

    fn on_record(
        &self,
        span: &span::Id,
        values: &span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.0.on_record(span, values, ctx.clone());
        self.1.on_record(span, values, ctx);
    }

    fn on_follows_from(
        &self,
        span: &span::Id,
        follows: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.0.on_follows_from(span, follows, ctx.clone());
        self.1.on_follows_from(span, follows, ctx);
    }

    fn event_enabled(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        let a_enabled = self.0.event_enabled(event, ctx.clone());
        let b_enabled = self.1.event_enabled(event, ctx);
        a_enabled || b_enabled
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.0.on_event(event, ctx.clone());
        self.1.on_event(event, ctx)
    }

    fn on_enter(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.0.on_enter(id, ctx.clone());
        self.1.on_enter(id, ctx);
    }

    fn on_exit(&self, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.0.on_exit(id, ctx.clone());
        self.1.on_exit(id, ctx);
    }

    fn on_close(&self, id: span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.0.on_close(id.clone(), ctx.clone());
        self.1.on_close(id, ctx);
    }

    fn on_id_change(
        &self,
        old: &span::Id,
        new: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.0.on_id_change(old, new, ctx.clone());
        self.1.on_id_change(old, new, ctx);
    }

    unsafe fn downcast_raw(&self, id: std::any::TypeId) -> Option<*const ()> {
        self.0.downcast_raw(id).or(self.1.downcast_raw(id))
    }
}
