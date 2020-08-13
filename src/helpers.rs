#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "base64helper")]
use std::borrow::{Borrow, ToOwned};

#[cfg(feature = "base64helper")]
pub struct Base64<T: ?Sized + Serialize + ToOwned>
where
	<T as ToOwned>::Owned: DeserializeOwned,
{
	_marker: std::marker::PhantomData<T>,
}
#[cfg(feature = "base64helper")]
impl<T: ?Sized + Serialize + ToOwned> crate::TextualSerde for Base64<T>
where
	<T as ToOwned>::Owned: DeserializeOwned,
{
	type A = <T as ToOwned>::Owned;
	type B = T;
	type Err = Result<data_encoding::DecodeError, bincode::Error>;
	fn serialize(x: &Self::B, out: &mut String) {
		// TODO: this allocation can be avoided by serializing directly into base64,
		// but that would require a lot more time to implement, since it would be on the
		// same scale as bincode.
		let serialized = bincode::serialize(x).expect("Could not serialize with bincode; this isn't supposed to happen");
		data_encoding::BASE64URL_NOPAD.encode_append(serialized.as_slice(), out)
	}
	fn deserialize(s: &str) -> Result<Self::A, Self::Err> {
		let serialized = data_encoding::BASE64URL_NOPAD.decode(s.as_bytes()).map_err(Ok)?;
		Ok(bincode::deserialize(&serialized).map_err(Err)?)
	}
	fn circular(x: &Self::A) -> &Self::B {
		x.borrow()
	}
}
