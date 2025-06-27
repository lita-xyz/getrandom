use crate::Error;
use core::mem::MaybeUninit;

pub use crate::util::{inner_u32, inner_u64};

extern "C" {
    fn valida_getrandom(buffer: *mut u8, length: usize);
}

#[inline]
pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    unsafe { valida_getrandom(dest.as_mut_ptr() as *mut u8, dest.len()) };
    Ok(())
}
