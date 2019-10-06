use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StreamForwarder<T> {
    upstream: T,
}

impl<T: futures_core::stream::Stream> Stream for StreamForwarder<T> {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.upstream.poll_next()
    }
}

fn main() {}
