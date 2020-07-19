#![allow(dead_code)]

#[test]
fn simple() {
	crate::path!(A: (parent: crate::RootPath) / "test" / (user: i64) / (item: i32));
	crate::path!(B: (parent: A) / "o" / (b: bool) / (uu: char));

	assert_eq!(A((&1i64, &2i32)).to_string(), "/test/1/2");
	assert_eq!(B((&3i64, &4i32, &true, &'a')).to_string(), "/test/3/4/o/true/a");
	assert_eq!(B::local((&true, &'a')).to_string(), "o/true/a");
}

#[cfg(feature = "warp")]
#[test]
fn warp() {
	use warp::{filters::BoxedFilter, Filter};

	crate::path!(A: (parent: crate::RootPath) / "test" / (user: i64) / (item: i32));
	crate::path!(B: (parent: A) / "o" / (b: bool) / (uu: char));

	let _a: BoxedFilter<(i64, i32)> = A::filter().boxed();
	let _b: BoxedFilter<(bool, char)> = B::filter().boxed();
}
