use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::info;

pub struct DumbFuture {}

impl Future for DumbFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        info!("Hello from a dumb future");
        unsafe {
            *(0xF00D as *mut u64) = 0x0;
        }
        unreachable!();
    }
}
