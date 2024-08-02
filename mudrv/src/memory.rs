use crate::{bindings::MUdeviceptr, Blob, CurrentCtx, Stream};
use context_spore::{impl_spore, AsRaw};
use std::{
    alloc::Layout,
    marker::PhantomData,
    mem::size_of_val,
    os::raw::c_void,
    ptr::null_mut,
    ops::{Deref, DerefMut},
    slice::{from_raw_parts, from_raw_parts_mut},
};

#[repr(transparent)]
pub struct DevByte(#[allow(unused)] u8);

#[inline]
pub fn memcpy_d2h<T: Copy>(dst: &mut [T], src: &[DevByte]) {
    let len = size_of_val(dst);
    let dst = dst.as_mut_ptr().cast();
    assert_eq!(len, size_of_val(src));
    mudrv!(muMemcpyDtoH_v2(dst, src.as_ptr() as _, len));
}

#[inline]
pub fn memcpy_h2d<T: Copy>(dst: &mut [DevByte], src: &[T]) {
    let len = size_of_val(src);
    let src = src.as_ptr().cast();
    assert_eq!(len, size_of_val(dst));
    mudrv!(muMemcpyHtoD_v2(dst.as_ptr() as _, src, len));
}

#[inline]
pub fn memcpy_d2d(dst: &mut [DevByte], src: &[DevByte]) {
    let len = size_of_val(src);
    assert_eq!(len, size_of_val(dst));
    mudrv!(muMemcpyDtoD_v2(dst.as_ptr() as _, src.as_ptr() as _, len));
}

impl Stream<'_> {
    #[inline]
    pub fn memcpy_h2d<T: Copy>(&self, dst: &mut [DevByte], src: &[T]) {
        let len = size_of_val(src);
        let src = src.as_ptr().cast();
        assert_eq!(len, size_of_val(dst));
        mudrv!(muMemcpyHtoDAsync_v2(
            dst.as_ptr() as _,
            src,
            len,
            self.as_raw()
        ));
    }

    #[inline]
    pub fn memcpy_d2d(&self, dst: &mut [DevByte], src: &[DevByte]) {
        let len = size_of_val(src);
        assert_eq!(len, size_of_val(dst));
        mudrv!(muMemcpyDtoDAsync_v2(
            dst.as_ptr() as _,
            src.as_ptr() as _,
            len,
            self.as_raw()
        ));
    }
}

impl_spore!(DevMem and DevMemSpore by (CurrentCtx, Blob<MUdeviceptr>));

impl CurrentCtx {
    pub fn malloc<T: Copy>(&self, len: usize) -> DevMem<'_> {
        let len = Layout::array::<T>(len).unwrap().size();
        let mut ptr = 0;
        mudrv!(muMemAlloc_v2(&mut ptr, len));
        DevMem(unsafe { self.wrap_raw(Blob { ptr, len }) }, PhantomData)
    }

    pub fn from_host<T: Copy>(&self, slice: &[T]) -> DevMem<'_> {
        let len = size_of_val(slice);
        let src = slice.as_ptr().cast();
        let mut ptr = 0;
        mudrv!(muMemAlloc_v2(&mut ptr, len));
        mudrv!(muMemcpyHtoD_v2(ptr, src, len));
        DevMem(unsafe { self.wrap_raw(Blob { ptr, len }) }, PhantomData)
    }
}

// impl<'ctx> Stream<'ctx> {
//     pub fn malloc<T: Copy>(&self, len: usize) -> DevMem<'ctx> {
//         let len = Layout::array::<T>(len).unwrap().size();
//         let mut ptr = 0;
//         mudrv!(muMemAllocAsync(&mut ptr, len, self.as_raw()));
//         DevMem(
//             unsafe { self.ctx().wrap_raw(Blob { ptr, len }) },
//             PhantomData,
//         )
//     }

//     pub fn from_host<T: Copy>(&self, slice: &[T]) -> DevMem<'ctx> {
//         let stream = unsafe { self.as_raw() };
//         let len = size_of_val(slice);
//         let src = slice.as_ptr().cast();
//         let mut ptr = 0;
//         mudrv!(muMemAllocAsync(&mut ptr, len, stream));
//         mudrv!(muMemcpyHtoDAsync_v2(ptr, src, len, stream));
//         DevMem(
//             unsafe { self.ctx().wrap_raw(Blob { ptr, len }) },
//             PhantomData,
//         )
//     }
// }

// impl DevMem<'_> {
//     #[inline]
//     pub fn drop_on(self, stream: &Stream) {
//         mudrv!(muMemFreeAsync(self.0.rss.ptr, stream.as_raw()));
//         forget(self);
//     }
// }

impl Drop for DevMem<'_> {
    #[inline]
    fn drop(&mut self) {
        mudrv!(muMemFree_v2(self.0.rss.ptr));
    }
}

impl Deref for DevMem<'_> {
    type Target = [DevByte];
    #[inline]
    fn deref(&self) -> &Self::Target {
        if self.0.rss.len == 0 {
            &[]
        } else {
            unsafe { from_raw_parts(self.0.rss.ptr as _, self.0.rss.len) }
        }
    }
}

impl DerefMut for DevMem<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.0.rss.len == 0 {
            &mut []
        } else {
            unsafe { from_raw_parts_mut(self.0.rss.ptr as _, self.0.rss.len) }
        }
    }
}

impl AsRaw for DevMemSpore {
    type Raw = MUdeviceptr;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0.rss.ptr
    }
}

impl DevMemSpore {
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.rss.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.rss.len == 0
    }
}


// use crate::{Blob, CurrentCtx};
// use context_spore::{impl_spore, AsRaw};
// use std::{
//     alloc::Layout,
//     marker::PhantomData,
//     ops::{Deref, DerefMut},
//     os::raw::c_void,
//     ptr::null_mut,
//     slice::{from_raw_parts, from_raw_parts_mut},
// };

impl_spore!(HostMem and HostMemSpore by (CurrentCtx, Blob<*mut c_void>));

impl CurrentCtx {
    pub fn malloc_host<T: Copy>(&self, len: usize) -> HostMem {
        let len = Layout::array::<T>(len).unwrap().size();
        let mut ptr = null_mut();
        mudrv!(muMemHostAlloc(&mut ptr, len, 0));
        HostMem(unsafe { self.wrap_raw(Blob { ptr, len }) }, PhantomData)
    }
}

impl Drop for HostMem<'_> {
    #[inline]
    fn drop(&mut self) {
        mudrv!(muMemFreeHost(self.0.rss.ptr));
    }
}

impl AsRaw for HostMem<'_> {
    type Raw = *mut c_void;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0.rss.ptr
    }
}

impl Deref for HostMem<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.0.rss.ptr.cast(), self.0.rss.len) }
    }
}

impl DerefMut for HostMem<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.0.rss.ptr.cast(), self.0.rss.len) }
    }
}

impl Deref for HostMemSpore {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.0.rss.ptr.cast(), self.0.rss.len) }
    }
}

impl DerefMut for HostMemSpore {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.0.rss.ptr.cast(), self.0.rss.len) }
    }
}

#[test]
fn test_behavior() {
    if let Err(crate::NoDevice) = crate::init() {
        return;
    }
    let mut ptr = null_mut();
    crate::Device::new(0).context().apply(|_| {
        mudrv!(muMemHostAlloc(&mut ptr, 128, 0));
        mudrv!(muMemFreeHost(ptr));
    });
    ptr = null_mut();
    mudrv!(muMemFreeHost(ptr));
}