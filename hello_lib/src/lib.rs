use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn hello(p_from: *const c_char) {
	let from = unsafe { CStr::from_ptr(p_from).to_string_lossy().into_owned() };
	println!("Hello from {} to Rust!", from);
}

#[no_mangle]
pub extern fn callback(p_callback: extern "C" fn(*const c_char)) {
	let from_environment = CString::new("Rust").expect("CString::new failed");
	p_callback(from_environment.as_ptr());
}

#[cfg(test)]
mod tests {
	use std::ffi::CString;
	use crate::*;

	#[test]
	fn success() {
		assert!(true);
	}

	#[test]
	fn hello_ok() {
		let from_environment = CString::new("Rust").expect("CString::new failed");
		hello(from_environment.as_ptr());
	}

	#[test]
	fn callback_ok() {
		callback(hello);
	}
}
