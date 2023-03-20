// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Types that should only be used for testing!

use sp_core::{
	bls,
	crypto::{ByteArray, CryptoTypePublicPair, KeyTypeId, Pair},
	ecdsa, ed25519, sr25519,
};

use crate::{
	vrf::{make_transcript, VRFSignature, VRFTranscriptData},
	Error, Keystore, KeystorePtr,
};

use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

/// A keystore implementation usable in tests.
#[derive(Default)]
pub struct MemoryKeystore {
	/// `KeyTypeId` maps to public keys and public keys map to private keys.
	keys: Arc<RwLock<HashMap<KeyTypeId, HashMap<Vec<u8>, String>>>>,
}

impl MemoryKeystore {
	/// Creates a new instance of `Self`.
	pub fn new() -> Self {
		Self::default()
	}

	fn sr25519_key_pair(&self, id: KeyTypeId, pub_key: &sr25519::Public) -> Option<sr25519::Pair> {
		self.keys.read().get(&id).and_then(|inner| {
			inner.get(pub_key.as_slice()).map(|s| {
				sr25519::Pair::from_string(s, None).expect("`sr25519` seed slice is valid")
			})
		})
	}

	fn ed25519_key_pair(&self, id: KeyTypeId, pub_key: &ed25519::Public) -> Option<ed25519::Pair> {
		self.keys.read().get(&id).and_then(|inner| {
			inner.get(pub_key.as_slice()).map(|s| {
				ed25519::Pair::from_string(s, None).expect("`ed25519` seed slice is valid")
			})
		})
	}

	fn ecdsa_key_pair(&self, id: KeyTypeId, pub_key: &ecdsa::Public) -> Option<ecdsa::Pair> {
		self.keys.read().get(&id).and_then(|inner| {
			inner
				.get(pub_key.as_slice())
				.map(|s| ecdsa::Pair::from_string(s, None).expect("`ecdsa` seed slice is valid"))
		})
	}

	fn bls_key_pair(&self, id: KeyTypeId, pub_key: &bls::Public) -> Option<bls::Pair> {
		self.keys.read().get(&id).and_then(|inner| {
			inner
				.get(pub_key.as_slice())
				.map(|s| bls::Pair::from_string(s, None).expect("`bls` seed slice is valid"))
		})
	}
}

impl Keystore for MemoryKeystore {
	fn keys(&self, id: KeyTypeId) -> Result<Vec<CryptoTypePublicPair>, Error> {
		self.keys
			.read()
			.get(&id)
			.map(|map| {
				Ok(map.keys().fold(Vec::new(), |mut v, k| {
					v.push(CryptoTypePublicPair(sr25519::CRYPTO_ID, k.clone()));
					v.push(CryptoTypePublicPair(ed25519::CRYPTO_ID, k.clone()));
					v.push(CryptoTypePublicPair(ecdsa::CRYPTO_ID, k.clone()));
					v
				}))
			})
			.unwrap_or_else(|| Ok(vec![]))
	}

	fn sr25519_public_keys(&self, id: KeyTypeId) -> Vec<sr25519::Public> {
		self.keys
			.read()
			.get(&id)
			.map(|keys| {
				keys.values()
					.map(|s| {
						sr25519::Pair::from_string(s, None).expect("`sr25519` seed slice is valid")
					})
					.map(|p| p.public())
					.collect()
			})
			.unwrap_or_default()
	}

	fn sr25519_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> Result<sr25519::Public, Error> {
		match seed {
			Some(seed) => {
				let pair = sr25519::Pair::from_string(seed, None).map_err(|_| {
					Error::ValidationError("Generates an `sr25519` pair.".to_owned())
				})?;
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), seed.into());
				Ok(pair.public())
			},
			None => {
				let (pair, phrase, _) = sr25519::Pair::generate_with_phrase(None);
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), phrase);
				Ok(pair.public())
			},
		}
	}

	fn ed25519_public_keys(&self, id: KeyTypeId) -> Vec<ed25519::Public> {
		self.keys
			.read()
			.get(&id)
			.map(|keys| {
				keys.values()
					.map(|s| {
						ed25519::Pair::from_string(s, None).expect("`ed25519` seed slice is valid")
					})
					.map(|p| p.public())
					.collect()
			})
			.unwrap_or_default()
	}

	fn ed25519_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> Result<ed25519::Public, Error> {
		match seed {
			Some(seed) => {
				let pair = ed25519::Pair::from_string(seed, None).map_err(|_| {
					Error::ValidationError("Generates an `ed25519` pair.".to_owned())
				})?;
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), seed.into());
				Ok(pair.public())
			},
			None => {
				let (pair, phrase, _) = ed25519::Pair::generate_with_phrase(None);
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), phrase);
				Ok(pair.public())
			},
		}
	}

	fn ecdsa_public_keys(&self, id: KeyTypeId) -> Vec<ecdsa::Public> {
		self.keys
			.read()
			.get(&id)
			.map(|keys| {
				keys.values()
					.map(|s| {
						ecdsa::Pair::from_string(s, None).expect("`ecdsa` seed slice is valid")
					})
					.map(|p| p.public())
					.collect()
			})
			.unwrap_or_default()
	}

	fn ecdsa_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> Result<ecdsa::Public, Error> {
		match seed {
			Some(seed) => {
				let pair = ecdsa::Pair::from_string(seed, None)
					.map_err(|_| Error::ValidationError("Generates an `ecdsa` pair.".to_owned()))?;
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), seed.into());
				Ok(pair.public())
			},
			None => {
				let (pair, phrase, _) = ecdsa::Pair::generate_with_phrase(None);
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), phrase);
				Ok(pair.public())
			},
		}
	}

	fn insert(&self, id: KeyTypeId, suri: &str, public: &[u8]) -> Result<(), ()> {
		self.keys
			.write()
			.entry(id)
			.or_default()
			.insert(public.to_owned(), suri.to_string());
		Ok(())
	}

	fn has_keys(&self, public_keys: &[(Vec<u8>, KeyTypeId)]) -> bool {
		public_keys
			.iter()
			.all(|(k, t)| self.keys.read().get(t).and_then(|s| s.get(k)).is_some())
	}

	fn sign_with(
		&self,
		id: KeyTypeId,
		key: &CryptoTypePublicPair,
		msg: &[u8],
	) -> Result<Option<Vec<u8>>, Error> {
		use codec::Encode;

		match key.0 {
			ed25519::CRYPTO_ID => {
				let key_pair = self
					.ed25519_key_pair(id, &ed25519::Public::from_slice(key.1.as_slice()).unwrap());

				key_pair.map(|k| k.sign(msg).encode()).map(Ok).transpose()
			},
			sr25519::CRYPTO_ID => {
				let key_pair = self
					.sr25519_key_pair(id, &sr25519::Public::from_slice(key.1.as_slice()).unwrap());

				key_pair.map(|k| k.sign(msg).encode()).map(Ok).transpose()
			},
			ecdsa::CRYPTO_ID => {
				let key_pair =
					self.ecdsa_key_pair(id, &ecdsa::Public::from_slice(key.1.as_slice()).unwrap());

				key_pair.map(|k| k.sign(msg).encode()).map(Ok).transpose()
			},
			_ => Err(Error::KeyNotSupported(id)),
		}
	}

	fn sr25519_vrf_sign(
		&self,
		key_type: KeyTypeId,
		public: &sr25519::Public,
		transcript_data: VRFTranscriptData,
	) -> Result<Option<VRFSignature>, Error> {
		let transcript = make_transcript(transcript_data);
		let pair =
			if let Some(k) = self.sr25519_key_pair(key_type, public) { k } else { return Ok(None) };

		let (inout, proof, _) = pair.as_ref().vrf_sign(transcript);
		Ok(Some(VRFSignature { output: inout.to_output(), proof }))
	}

	fn ecdsa_sign_prehashed(
		&self,
		id: KeyTypeId,
		public: &ecdsa::Public,
		msg: &[u8; 32],
	) -> Result<Option<ecdsa::Signature>, Error> {
		let pair = self.ecdsa_key_pair(id, public);
		pair.map(|k| k.sign_prehashed(msg)).map(Ok).transpose()
	}

	fn bls_public_keys(&self, id: KeyTypeId) -> Vec<bls::Public> {
		self.keys
			.read()
			.get(&id)
			.map(|keys| {
				keys.values()
					.map(|s| bls::Pair::from_string(s, None).expect("`bls` seed slice is valid"))
					.map(|p| p.public())
					.collect()
			})
			.unwrap_or_default()
	}

	fn bls_generate_new(&self, id: KeyTypeId, seed: Option<&str>) -> Result<bls::Public, Error> {
		match seed {
			Some(seed) => {
				let pair = bls::Pair::from_string(seed, None)
					.map_err(|_| Error::ValidationError("Generates an `bls` pair.".to_owned()))?;
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), seed.into());
				Ok(pair.public())
			},
			None => {
				let (pair, phrase, _) = bls::Pair::generate_with_phrase(None);
				self.keys
					.write()
					.entry(id)
					.or_default()
					.insert(pair.public().to_raw_vec(), phrase);
				Ok(pair.public())
			},
		}
	}

	fn bls_sign(
		&self,
		id: KeyTypeId,
		public: &bls::Public,
		msg: &[u8],
	) -> Result<Option<bls::Signature>, Error> {
		let pair = self.bls_key_pair(id, public);
		pair.map(|k| k.sign(msg)).map(Ok).transpose()
	}
}

impl Into<KeystorePtr> for MemoryKeystore {
	fn into(self) -> KeystorePtr {
		Arc::new(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::vrf::VRFTranscriptValue;
	use sp_core::{
		sr25519,
		testing::{ECDSA, ED25519, SR25519},
	};

	#[test]
	fn store_key_and_extract() {
		let store = MemoryKeystore::new();

		let public = store.ed25519_generate_new(ED25519, None).expect("Generates key");

		let public_keys = store.keys(ED25519).unwrap();

		assert!(public_keys.contains(&public.into()));
	}

	#[test]
	fn store_unknown_and_extract_it() {
		let store = MemoryKeystore::new();

		let secret_uri = "//Alice";
		let key_pair = sr25519::Pair::from_string(secret_uri, None).expect("Generates key pair");

		store
			.insert(SR25519, secret_uri, key_pair.public().as_ref())
			.expect("Inserts unknown key");

		let public_keys = store.keys(SR25519).unwrap();

		assert!(public_keys.contains(&key_pair.public().into()));
	}

	#[test]
	fn vrf_sign() {
		let store = MemoryKeystore::new();

		let secret_uri = "//Alice";
		let key_pair = sr25519::Pair::from_string(secret_uri, None).expect("Generates key pair");

		let transcript_data = VRFTranscriptData {
			label: b"Test",
			items: vec![
				("one", VRFTranscriptValue::U64(1)),
				("two", VRFTranscriptValue::U64(2)),
				("three", VRFTranscriptValue::Bytes("test".as_bytes().to_vec())),
			],
		};

		let result = store.sr25519_vrf_sign(SR25519, &key_pair.public(), transcript_data.clone());
		assert!(result.unwrap().is_none());

		store
			.insert(SR25519, secret_uri, key_pair.public().as_ref())
			.expect("Inserts unknown key");

		let result = store.sr25519_vrf_sign(SR25519, &key_pair.public(), transcript_data);

		assert!(result.unwrap().is_some());
	}

	#[test]
	fn ecdsa_sign_prehashed_works() {
		let store = MemoryKeystore::new();

		let suri = "//Alice";
		let pair = ecdsa::Pair::from_string(suri, None).unwrap();

		let msg = sp_core::keccak_256(b"this should be a hashed message");

		// no key in key store
		let res = store.ecdsa_sign_prehashed(ECDSA, &pair.public(), &msg).unwrap();
		assert!(res.is_none());

		// insert key, sign again
		store.insert(ECDSA, suri, pair.public().as_ref()).unwrap();

		let res = store.ecdsa_sign_prehashed(ECDSA, &pair.public(), &msg).unwrap();
		assert!(res.is_some());
	}
}
