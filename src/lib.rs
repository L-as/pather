#[doc(hidden)]
pub use std::fmt::Display;
#[doc(hidden)]
pub use std::marker::PhantomData;
#[cfg(feature = "warp")]
#[doc(hidden)]
pub use warp;

#[cfg(test)]
mod tests;

pub mod helpers;

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct Product<H, T: HList>(pub H, pub T);

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct HNil;

#[doc(hidden)]
// Converts Product (and ()) into tuples.
pub trait HList: Sized {
	type Tuple: Tuple<HList = Self>;

	fn flatten(self) -> Self::Tuple;
}

#[doc(hidden)]
// Typeclass that tuples can be converted into a Product (or unit ()).
pub trait Tuple: Sized {
	type HList: HList<Tuple = Self>;

	fn hlist(self) -> Self::HList;
}

#[doc(hidden)]
// Combines Product together.
pub trait Combine<T: HList> {
	type Output: HList;

	fn combine(self, other: T) -> Self::Output;
}

impl<T: HList> Combine<T> for HNil {
	type Output = T;
	#[inline]
	fn combine(self, other: T) -> Self::Output {
		other
	}
}

impl<H, T: HList, U: HList> Combine<U> for Product<H, T>
where
	T: Combine<U>,
	Product<H, <T as Combine<U>>::Output>: HList,
{
	type Output = Product<H, <T as Combine<U>>::Output>;

	#[inline]
	fn combine(self, other: U) -> Self::Output {
		Product(self.0, self.1.combine(other))
	}
}

#[doc(hidden)]
pub trait ReverseInto<T: HList> {
	type Output: HList;

	fn reverse_into(self, other: T) -> Self::Output;
}

impl<T: HList> ReverseInto<T> for HNil {
	type Output = T;
	#[inline]
	fn reverse_into(self, other: T) -> Self::Output {
		other
	}
}

impl<H, T: HList, U: HList> ReverseInto<U> for Product<H, T>
where
	Product<H, U>: HList,
	T: ReverseInto<Product<H, U>>,
	<T as ReverseInto<Product<H, U>>>::Output: HList,
{
	type Output = <T as ReverseInto<Product<H, U>>>::Output;

	#[inline]
	fn reverse_into(self, other: U) -> Self::Output {
		self.1.reverse_into(Product(self.0, other))
	}
}

impl HList for HNil {
	type Tuple = ();

	#[inline]
	fn flatten(self) -> Self::Tuple {
		()
	}
}

impl Tuple for () {
	type HList = HNil;

	#[inline]
	fn hlist(self) -> Self::HList {
		HNil
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! product {
	() => { $crate::HNil };
	($H:expr) => { $crate::Product($H, $crate::HNil) };
	($H:expr, $($T:expr),*) => { $crate::Product($H, $crate::product!($($T),*)) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! Product {
	() => { $crate::HNil };
	($H:ty) => { $crate::Product<$H, $crate::HNil> };
	($H:ty, $($T:ty),*) => { $crate::Product<$H, $crate::Product!($($T),*)> };
}

#[doc(hidden)]
#[macro_export]
macro_rules! product_pat {
	($H:pat) => { Product($H, HNil) };
	($H:pat, $($T:pat),*) => { Product($H, product_pat!($($T),*)) };
}

macro_rules! generics {
	($type:ident) => {
		impl<$type> HList for Product!($type) {
			type Tuple = ($type,);

			#[inline]
			fn flatten(self) -> Self::Tuple {
				(self.0,)
			}
		}

		impl<$type> Tuple for ($type,) {
			type HList = Product!($type);
			#[inline]
			fn hlist(self) -> Self::HList {
				product!(self.0)
			}
		}
	};

	($type1:ident, $( $type:ident ),*) => {
		generics!($( $type ),*);

		impl<$type1, $( $type ),*> HList for Product!($type1, $($type),*) {
			type Tuple = ($type1, $( $type ),*);

			#[inline]
			fn flatten(self) -> Self::Tuple {
				#[allow(non_snake_case)]
				let product_pat!($type1, $( $type ),*) = self;
				($type1, $( $type ),*)
			}
		}

		impl<$type1, $( $type ),*> Tuple for ($type1, $($type),*) {
			type HList = Product!($type1, $( $type ),*);

			#[inline]
			fn hlist(self) -> Self::HList {
				#[allow(non_snake_case)]
				let ($type1, $( $type ),*) = self;
				product!($type1, $( $type ),*)
			}
		}
	};
}

generics! {
	T1,
	T2,
	T3,
	T4,
	T5,
	T6,
	T7,
	T8,
	T9,
	T10,
	T11,
	T12,
	T13,
	T14,
	T15,
	T16
}

#[cfg(feature = "warp")]
#[macro_export]
macro_rules! path_filter {
	(/ $($tail:tt)*) => ($crate::path_filter!($($tail)*));
	(@segment $next:literal) => ({
		#[derive(Clone, Copy)]
		struct __StaticPath;
		impl ::std::convert::AsRef<str> for __StaticPath {
			fn as_ref(&self) -> &str {
				static S: &str = $next;
				S
			}
		}
		$crate::warp::path(__StaticPath)
	});
	(@segment ($next:ident : $ty:path)) => (
		// FIXME: does this do percent decoding?
		(warp::Filter::and_then($crate::warp::path::param::<String>(), |x: String| {async move {<$ty as $crate::TextualSerde>::deserialize(&x).map_err(|_| $crate::warp::reject::not_found())}}))
	);
	($next:tt) => (
		$crate::warp::Filter::and($crate::path_filter!(@segment $next), $crate::warp::path::end())
	);
	($next:tt / ..) => (
		$crate::path_filter!(@segment $next)
	);
	($next:tt / $nextnext:tt $(/ $tail:tt)*) => (
		$crate::warp::Filter::and($crate::path_filter!(@segment $next), $crate::path_filter!($nextnext $(/ $tail)*))
	);
}

pub trait Path {
	type Params: HList;
}

pub struct RootPath<'a> {
	_marker: PhantomData<&'a ()>,
}
impl<'a> Path for RootPath<'a> {
	type Params = HNil;
}
#[allow(non_snake_case)]
pub fn RootPath<'a>(_params: <<<RootPath<'a> as Path>::Params as ReverseInto<HNil>>::Output as HList>::Tuple) -> String {
	String::new()
}

use std::{
	borrow::{Borrow, ToOwned},
	fmt::Write,
	str::FromStr,
};

pub trait TextualSerde {
	type A;
	type B: ?Sized;
	type Err;
	fn serialize(x: &Self::B, out: &mut String);
	fn deserialize(s: &str) -> Result<Self::A, Self::Err>;
	fn circular(x: &Self::A) -> &Self::B;
}

pub struct NoClone<T> {
	_marker: PhantomData<T>,
}

impl<T: Display + FromStr> TextualSerde for NoClone<T> {
	type A = T;
	type B = T;
	type Err = <Self::A as FromStr>::Err;
	fn serialize(x: &Self::B, out: &mut String) {
		// FIXME: This is inefficient, but I am not sure there is a better way.
		// The entire std::fmt module honestly just seems like bad design. but
		// maybe there still is a way to bypass the whole formatting/arguments
		// shenanigans.
		// Same problem exists below.
		write!(out, "{}", x).expect("std::fmt::Display::fmt failed")
	}
	fn deserialize(s: &str) -> Result<Self::A, Self::Err> {
		Self::A::from_str(s)
	}
	fn circular(x: &Self::A) -> &Self::B {
		x.borrow()
	}
}

impl<B: Display + ?Sized + ToOwned> TextualSerde for B
where
	<B as ToOwned>::Owned: FromStr,
{
	type A = <B as ToOwned>::Owned;
	type B = B;
	type Err = <Self::A as FromStr>::Err;
	fn serialize(x: &Self::B, out: &mut String) {
		// FIXME: same problem as above
		write!(out, "{}", x).expect("std::fmt::Display::fmt failed")
	}
	fn deserialize(s: &str) -> Result<Self::A, Self::Err> {
		Self::A::from_str(s)
	}
	fn circular(x: &Self::A) -> &Self::B {
		x.borrow()
	}
}

#[doc(hidden)]
#[cfg(feature = "warp")]
#[macro_export]
macro_rules! if_warp {
	($($x:tt)*) => ($($x)*)
}

#[doc(hidden)]
#[cfg(not(feature = "warp"))]
#[macro_export]
macro_rules! if_warp {
	($($x:tt)*) => {};
}

#[macro_export]
macro_rules! path {
	(@conv [$head:ty $(,$tail:ty)*] [$expr:expr]) => {
		$crate::path!(@conv [$($tail),*] [($expr).1])
	};
	(@conv [] [$expr:expr]) => {
		($expr)
	};
	(@revslice [$head:ty $(,$tail:ty)*] [$($result:expr),*] [$expr:expr]) => {
		$crate::path!(@revslice [$($tail),*] [($expr).0 $(,$result)*] [($expr).1])
	};
	(@revslice [] [$($result:expr),*] [$expr:expr]) => {
		$crate::product!($($result),*)
	};
	(@format [$e:ident] [$s:expr] / $($tail:tt)*) => ($crate::path!(@format [$e] [$s] $($tail)*));
	(@format [$e:ident] [$s:expr] @segment $next:literal) => ({
		$s.push_str($next);
	});
	(@format [$e:ident] [$s:expr] @segment ($next:ident : $ty:ty)) => (
		// FIXME: do percent-encoding
		<$ty as $crate::TextualSerde>::serialize($e.0, &mut $s);
		#[allow(unused_variables)]
		let $e = $e.1;
	);
	(@format [$e:ident] [$s:expr] $next:tt $(/ ..)?) => (
		$crate::path!(@format [$e] [$s] @segment $next);
	);
	(@format [$e:ident] [$s:expr] $next:tt / $nextnext:tt $(/ $tail:tt)*) => (
		$crate::path!(@format [$e] [$s] @segment $next);
		$s.push('/');
		$crate::path!(@format [$e] [$s] $nextnext $(/ $tail)*);
	);
	(@munch [$($state:tt)*] [$($format:literal)*] [$($tyrev:ty),*] [$($ty:ty),*] [$head:literal $(,$tail:tt)*]) => (
		$crate::path!(@munch [$($state)*] [$($format)* $head] [$($tyrev),*] [$($ty),*] [$($tail),*]);
	);
	(@munch [$($state:tt)*] [$($format:literal)*] [$($tyrev:ty),*] [$($ty:ty),*] [($head:ident: $headty:ty) $(,$tail:tt)*]) => (
		$crate::path!(@munch [$($state)*] [$($format)* "{}"] [$headty $(,$tyrev)*] [$($ty,)* $headty] [$($tail),*]);
	);
	(@munch [$vis:vis $name:ident [$($parent:tt)*] [$($original_input:tt)*]] [$($format:literal)*] [$($tyrev:ty),*] [$($ty:ty),*] [$(..)?]) => (
		$vis struct $name<'a> {
			_marker: $crate::PhantomData<&'a ()>,
		}
		impl<'a> $crate::Path for $name<'a> {
			type Params = <<$crate::Product!($(&'a <$ty as $crate::TextualSerde>::B),*) as $crate::ReverseInto<$crate::HNil>>::Output as $crate::Combine<<$($parent)*<'a> as $crate::Path>::Params>>::Output;
		}
		impl<'a> $name<'a> {
			$crate::if_warp! {
				pub(self) fn filter() -> impl $crate::warp::Filter<Extract = ($(<$ty as $crate::TextualSerde>::A,)*), Error = $crate::warp::Rejection> + Clone {
					$crate::path_filter!($($original_input)*)
				}
			}
			#[allow(dead_code)]
			pub(self) fn local(params: <$crate::Product!($(&'a <$ty as $crate::TextualSerde>::B),*) as $crate::HList>::Tuple) -> String {
				#[allow(unused_variables)]
				let params = $crate::Tuple::hlist(params);
				let mut s = String::new();
				$crate::path!(@format [params] [s] $($original_input)*);
				s
			}
		}
		#[allow(non_snake_case)]
		$vis fn $name<'a>(params: <<<$name<'a> as $crate::Path>::Params as $crate::ReverseInto<$crate::HNil>>::Output as $crate::HList>::Tuple) -> String {
			let params: <$name<'a> as $crate::Path>::Params = $crate::ReverseInto::reverse_into($crate::Tuple::hlist(params), $crate::HNil);
			let parent_params: <$($parent)*<'a> as $crate::Path>::Params = $crate::path!(@conv [$($ty),*] [params]);
			let parent_params: <<<$($parent)*<'a> as $crate::Path>::Params as $crate::ReverseInto<$crate::HNil>>::Output as $crate::HList>::Tuple = $crate::HList::flatten($crate::ReverseInto::reverse_into(parent_params, $crate::HNil));
			#[allow(unused_mut)]
			let mut s = ($($parent)*)(parent_params);
			s.reserve(32); // TODO: make this dependent on the input
			s.push('/');
			#[allow(unused_variables)]
			let params = $crate::path!(@revslice [$($ty),*] [] [params]);
			$crate::path!(@format [params] [s] $($original_input)*);
			s
		}
	);

	($vis:vis $name:ident: (parent: $($parent:tt)*) $(/ $path:tt)*) => (
		$crate::path!(@munch [$vis $name [$($parent)*] [$(/ $path)*]] [] [] [] [$($path),*]);
	);
}
