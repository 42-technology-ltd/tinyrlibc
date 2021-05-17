//! Rust implementation of C library function `strncpy`
//!
//! Copyright (c) 42 Technology 2021
//! Licensed under the Blue Oak Model Licence 1.0.0

use crate::CChar;

/// Rust implementation of C library function `strncpy`. Passing NULL
/// (core::ptr::null()) gives undefined behaviour.
#[no_mangle]
pub unsafe extern "C" fn strncpy(dest: *mut CChar, source: *const CChar, n: usize) -> *mut CChar {
	let mut found_end = false;
	for i in 0..n as isize {	
		let s = if found_end { 0 } else { *source.offset(i) };
		*dest.offset(i) = s;

		if s == 0
		{
			found_end = true;
		}

	}
	dest
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn short_copy() {
		let a = b"123\0";
		let mut b = [0xCCu8; 8];
		let result = unsafe { strncpy(b.as_mut_ptr(), a.as_ptr(), 3) };
		assert_eq!(b.as_mut_ptr(), result);
		assert_eq!(b'1', b[0]);
		assert_eq!(b'2', b[1]);
		assert_eq!(b'3', b[2]);
		assert_eq!(0xCC, b[3]);
	}

	#[test]
	fn long_copy() {
		let a = b"123456789\0";
		let mut b = [0xCCu8; 4];
		let result = unsafe { strncpy(b.as_mut_ptr(), a.as_ptr(), 4) };
		assert_eq!(b.as_mut_ptr(), result);
		assert_eq!(b'1', b[0]);
		assert_eq!(b'2', b[1]);
		assert_eq!(b'3', b[2]);
		assert_eq!(b'4', b[3]);
	}

}
