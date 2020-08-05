#![allow(dead_code)]

#[cfg(feature = "warp")]
use warp::{filters::BoxedFilter, Filter};

#[cfg(feature = "base64helper")]
use crate::helpers::Base64;

crate::path!(pub A: (parent: crate::RootPath) / "test" / (user: i64) / (item: i32) / ..);
crate::path!(pub B: (parent: A) / "o" / (b: bool) / (uu: char));
crate::path!(pub C: (parent: crate::RootPath) / "c");

#[test]
fn top() {
	assert_eq!(A((&1i64, &2i32)), "/test/1/2");
	assert_eq!(B((&3i64, &4i32, &true, &'a')), "/test/3/4/o/true/a");
	assert_eq!(B::local((&true, &'a')), "o/true/a");
	assert_eq!(C(()), "/c");
	assert_eq!(C::local(()), "c");
}

mod t {
	#[cfg(feature = "warp")]
	use warp::{filters::BoxedFilter, Filter};

	crate::path!(pub D: (parent: super::C) / "dju" / (s: str));
	#[test]
	fn bottom() {
		assert_eq!(D(("test",)), "/c/dju/test");

		#[cfg(feature = "warp")]
		let _d: BoxedFilter<(String,)> = D::filter().boxed();
	}
}

crate::path!(pub E: (parent: self::t::D) / "hhh" / (data: Base64<[u8]>));
#[cfg(feature = "base64helper")]
crate::path!(pub F: (parent: crate::RootPath) / (int: Base64<u128>));

#[test]
fn base64() {
	assert_eq!(E(("test", &[1, 2, 3, 4, 5, 6, 7, 8][..],)), "/c/dju/test/hhh/CAAAAAAAAAABAgMEBQYHCA");
	#[cfg(feature = "base64helper")]
	assert_eq!(F((&4210,)), "/chAAAAAAAAAAAAAAAAAAAA");
	#[cfg(feature = "base64helper")]
	assert_eq!(4205, <Base64<u128> as crate::TextualSerde>::deserialize("bRAAAAAAAAAAAAAAAAAAAAAP").unwrap());
}

#[cfg(feature = "warp")]
#[test]
fn warp() {
	// TODO: verify that these filters work
	let _a: BoxedFilter<(i64, i32)> = A::filter().boxed();
	let _b: BoxedFilter<(bool, char)> = B::filter().boxed();
	#[cfg(feature = "base64helper")]
	let _f: BoxedFilter<(u128,)> = F::filter().boxed();
}
