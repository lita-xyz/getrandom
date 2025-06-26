use crate::Error;
use core::mem::MaybeUninit;

extern "C" {
    fn valida_getrandom(buffer: *mut u8, length: usize);
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    unsafe { valida_getrandom(dest.as_mut_ptr() as *mut u8, dest.len()) };

    Ok(())
}
