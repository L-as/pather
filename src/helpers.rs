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
	type Err = Result<base64::DecodeError, bincode::Error>;
	fn serialize(x: &Self::B, out: &mut String) {
		// TODO: this allocation can be avoided by serializing directly into base64,
		// but that would require a lot more time to implement, since it would be on the
		// same scale as bincode.
		let serialized = bincode::serialize(x).expect("Could not serialize with bincode");
		base64::encode_config_buf(serialized, base64::URL_SAFE_NO_PAD, out);
	}
	fn deserialize(s: &str) -> Result<Self::A, Self::Err> {
		let serialized = base64::decode_config(s, base64::URL_SAFE_NO_PAD).unwrap();
		Ok(bincode::deserialize(&serialized).unwrap())
	}
	fn circular(x: &Self::A) -> &Self::B {
		x.borrow()
	}
}
