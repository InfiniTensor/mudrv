use crate::{bindings::MUstream, CurrentCtx};
use context_spore::{impl_spore, AsRaw};
use std::{marker::PhantomData, ptr::null_mut};

impl_spore!(Stream and StreamSpore by (CurrentCtx, MUstream));

impl CurrentCtx {
    #[inline]
    pub fn stream(&self) -> Stream {
        let mut stream = null_mut();
        mudrv!(muStreamCreate(&mut stream, 0));
        Stream(unsafe { self.wrap_raw(stream) }, PhantomData)
    }
}

impl Drop for Stream<'_> {
    #[inline]
    fn drop(&mut self) {
        self.synchronize();
        mudrv!(muStreamDestroy_v2(self.0.rss));
    }
}

impl AsRaw for Stream<'_> {
    type Raw = MUstream;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0.rss
    }
}

impl Stream<'_> {
    #[inline]
    pub fn synchronize(&self) {
        mudrv!(muStreamSynchronize(self.0.rss));
    }
}