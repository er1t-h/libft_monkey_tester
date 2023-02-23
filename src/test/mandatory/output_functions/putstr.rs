use std::{fs::File, os::unix::prelude::AsRawFd};
use std::ffi::CString;

macro_rules! test {
	($name: ident, $to_write: expr) => {
		crate::fork_test! {
			#[test]
			fn $name() {
				let filename = format!(".tests_putstr/{}.txt", line!());
				let file = File::create(&filename).unwrap();
				let fd = file.as_raw_fd();
				let to_print = CString::new($to_write).unwrap();
				unsafe { crate::ft_putstr_fd(to_print.as_ptr(), fd) }
				let content = std::fs::read_to_string(filename).unwrap();
				assert_eq!(content, $to_write);
			}
		}
	};
}

test!(basic, "Super !");
test!(longer, "En vrai faire un call a write pour chaque caractere c'est pas ouf");
test!(utf8, "Salut! C'est un test de qualité contenant de supers UTF-8. 🀄麻雀🀄がしたい。このテストは本当に面白いなぁ。");

crate::fork_test! {
	#[test]
	fn null() {
		let filename = format!(".tests_putstr/{}.txt", line!());
		let file = File::create(&filename).unwrap();
		let fd = file.as_raw_fd();
		unsafe { crate::ft_putstr_fd(std::ptr::null(), fd) }
	}
}
