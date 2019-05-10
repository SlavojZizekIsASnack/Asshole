mod small;
pub use small::Small;

#[repr(C)]
struct CVec<T> {
	ptr: *mut T,
	len: libc::size_t,
}

impl<T> CVec<T> {
	fn as_cvec(vec: &mut Vec<T>) -> CVec<T> {
		CVec {
			ptr: vec.as_mut_ptr(),
			len: vec.len(),
		}
	}
}

#[repr(C)]
struct Play {
	start: libc::c_uint,
	length: libc::c_uint,
}

impl Play {
	fn into_vec(self) -> Vec<libc::c_uint> {
		let mut vec = vec![];
		for i in 0..self.start {
			vec.push(self.start + i);
		}

		vec
	}
}