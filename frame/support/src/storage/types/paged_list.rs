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

//! Storage map type. Implements StorageMap, StorageIterableMap, StoragePrefixedMap traits and their
//! methods directly.

#![allow(unused_imports)] // FAIL-CI

use crate::{
	assert_storage_noop, defensive,
	storage::{KeyLenOf, StorageAppend, StorageDecodeLength, StoragePrefixedMap, StorageTryAppend},
	traits::{Defensive, Get, GetDefault, StorageInfo, StorageInstance},
	CloneNoBound, DebugNoBound, DefaultNoBound, EqNoBound, PartialEqNoBound, StorageHasher,
	StorageNoopGuard, Twox128,
};
use codec::{Decode, Encode, EncodeLike, FullCodec, MaxEncodedLen};
use core::iter::Peekable;
use sp_arithmetic::traits::{SaturatedConversion, Saturating};
use sp_std::prelude::*;

pub type PageIndex = u32;
pub type ValueIndex = u32;

/// A paginated storage list that packs multiple elements per page.
///
/// The map is lined by a storage lookup of `(prefix, page_index)`. See [`StoragePagedListMeta`].
///
/// Supported APIs are iteration and appending.
pub struct StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages = GetDefault> {
	_phantom: core::marker::PhantomData<(Prefix, Hasher, Value, ValuesPerPage, MaxPages)>,
}

/// State info for book-keeping. Can be used as append-only iterator.
#[derive(
	Encode, Decode, CloneNoBound, PartialEqNoBound, EqNoBound, DebugNoBound, DefaultNoBound,
)]
// todo ignore scale bounds
pub struct StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage> {
	first_page: PageIndex,   // can be >0 when pages were delete.
	first_value: ValueIndex, // inside `first_page`

	current_page: PageIndex,   // next one is free, always >= first_page
	current_value: ValueIndex, // inside `current_page`

	_phantom: core::marker::PhantomData<(Prefix, Value, Hasher, ValuesPerPage)>,
}

impl<Prefix, Value, Hasher, ValuesPerPage> crate::storage::StorageAppendix<Value>
	for StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	fn append<EncodeLikeValue>(&mut self, item: EncodeLikeValue)
	where
		EncodeLikeValue: EncodeLike<Value>,
	{
		self.append_one(item);
	}
}

impl<Prefix, Value, Hasher, ValuesPerPage>
	StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	pub fn from_storage() -> Option<Self> {
		let key = Self::key();

		if let Some(raw) = sp_io::storage::get(key.as_ref()) {
			Self::decode(&mut &raw[..]).ok()
		} else {
			None
		}
	}

	pub fn key() -> Hasher::Output {
		(Prefix::STORAGE_PREFIX, b"paged_list_state").using_encoded(Hasher::hash)
	}

	pub fn append_one<EncodeLikeValue>(&mut self, item: EncodeLikeValue)
	where
		EncodeLikeValue: EncodeLike<Value>,
	{
		// Note: we use >= here in case someone decreased it in a runtime upgrade.
		if self.current_value >= ValuesPerPage::get() {
			self.current_value = 0;
			self.current_page.saturating_inc();
		}
		let key = page_key::<Prefix, Hasher>(self.current_page);
		self.current_value.saturating_inc();
		log::info!("Appending to page/val {}/{}", self.current_page, self.current_value);
		// Pages are `Vec` and therefore appendable.
		sp_io::storage::append(key.as_ref(), item.encode());
		self.store();
	}

	pub fn store(&self) {
		let key = Self::key();
		log::info!("Storing: {:?}", &self);
		self.using_encoded(|encoded| sp_io::storage::set(key.as_ref(), encoded));
	}

	pub fn reset(&mut self) {
		*self = Default::default();
		Self::clear();
	}

	pub fn clear() {
		let key = Self::key();
		sp_io::storage::clear(key.as_ref());
	}
}

// invariant: empty pages do not exist
pub struct Page<V> {
	current: PageIndex, // current value for iteration
	index: PageIndex,
	values: sp_std::vec::Vec<V>,
}

impl<V: FullCodec> Page<V> {
	fn from_storage<Prefix: StorageInstance, Hasher: StorageHasher>(
		index: PageIndex,
	) -> Option<Self> {
		let key = page_key::<Prefix, Hasher>(index);
		let values = sp_io::storage::get(key.as_ref())
			.map(|raw| sp_std::vec::Vec::<V>::decode(&mut &raw[..]).ok())
			.flatten();
		let values = values.unwrap_or_default();
		if values.is_empty() {
			None
		} else {
			Some(Self { current: 0, index, values })
		}
	}

	pub fn delete<Prefix: StorageInstance, Hasher: StorageHasher>(&self) {
		delete_page::<Prefix, Hasher>(self.index);
	}
}

pub(crate) fn delete_page<Prefix: StorageInstance, Hasher: StorageHasher>(index: PageIndex) {
	let key = page_key::<Prefix, Hasher>(index);
	sp_io::storage::clear(key.as_ref());
}

// Note: does not live under `Page` since it does not require `Value`.
pub(crate) fn page_key<Prefix: StorageInstance, Hasher: StorageHasher>(
	index: PageIndex,
) -> Hasher::Output {
	(Prefix::STORAGE_PREFIX, index).using_encoded(Hasher::hash)
}

impl<V: Clone> Iterator for Page<V> {
	type Item = V;

	fn next(&mut self) -> Option<Self::Item> {
		let val = self.values.get(self.current as usize).cloned();
		self.current.saturating_inc();
		val
	}
}

pub struct StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage> {
	// Design: we put the Page into the iterator to have fewer storage look-ups. Yes, these
	// look-ups would be cached anyway, but bugging the overlay on each `.next` call still seems
	// like a poor trade-off than caching it in the iterator directly. Note: if Page is empty then
	// the iterator did not find any data upon setup or ran out of pages.
	page: Option<Page<Value>>,
	drain: bool,
	meta: StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>,
	_phantom: core::marker::PhantomData<(Prefix, Value, Hasher, ValuesPerPage)>,
}

impl<Prefix, Value, Hasher, ValuesPerPage>
	StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	pub fn from_meta(
		meta: StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>,
		drain: bool,
	) -> Self {
		let mut page = Page::<Value>::from_storage::<Prefix, Hasher>(meta.first_page);

		if let Some(ref mut page) = page {
			page.current = meta.first_value;
		}

		Self { page, drain, meta, _phantom: Default::default() }
	}
}

impl<Prefix, Value, Hasher, ValuesPerPage> Iterator
	for StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
{
	type Item = Value;

	fn next(&mut self) -> Option<Self::Item> {
		let Some(ref mut page) = self.page else {
			return None;
		};
		if let Some(val) = page.next() {
			if self.drain {
				self.meta.first_value.saturating_inc();
				self.meta.store()
			}
			return Some(val)
		}
		if self.drain {
			// page is empty
			page.delete::<Prefix, Hasher>();
			self.meta.first_value = 0;
			self.meta.first_page.saturating_inc();
			self.meta.store();
		}

		let Some(mut page) = Page::from_storage::<Prefix, Hasher>(page.index.saturating_add(1)) else {
			self.page = None;
			if self.drain {
				self.meta.reset();
			}
			return None;
		};

		if let Some(val) = page.next() {
			self.page = Some(page);
			if self.drain {
				self.meta.first_value.saturating_inc();
				self.meta.store();
			}
			return Some(val)
		}

		defensive!("StoragePagedListIterator: empty pages do not exist: storage corrupted");
		// Put the storage back into a consistent state.
		self.meta.reset();
		self.page = None;
		None
	}
}

impl<Prefix, Hasher, Value, ValuesPerPage, MaxPages> crate::storage::StoragePagedList<Value>
	for StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
	MaxPages: Get<Option<u32>>,
{
	type Iterator = StoragePagedListIterator<Prefix, Value, Hasher, ValuesPerPage>;
	type Appendix = StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>;

	fn iter() -> Self::Iterator {
		StoragePagedListIterator::from_meta(Self::read_meta(), false)
	}

	fn drain() -> Self::Iterator {
		StoragePagedListIterator::from_meta(Self::read_meta(), true)
	}

	fn appendix() -> Self::Appendix {
		Self::appendix()
	}
}

#[cfg(feature = "std")]
impl<Prefix, Hasher, Value, ValuesPerPage, MaxPages> crate::storage::TestingStoragePagedList<Value>
	for StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
	MaxPages: Get<Option<u32>>,
{
	type Metadata = StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage>;

	fn read_meta() -> Option<Self::Metadata> {
		Self::Metadata::from_storage()
	}

	fn clear_meta() {
		Self::Metadata::clear()
	}
}

impl<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
	StoragePagedList<Prefix, Hasher, Value, ValuesPerPage, MaxPages>
where
	Prefix: StorageInstance,
	Value: FullCodec + Clone,
	Hasher: StorageHasher,
	ValuesPerPage: Get<u32>,
	MaxPages: Get<Option<u32>>,
{
	pub(crate) fn read_meta() -> StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage> {
		// Use default here to not require a setup migration.
		StoragePagedListMeta::from_storage().unwrap_or_default()
	}

	/// Provides a fast append iterator.
	///
	/// The list should not be modified while appending. Also don't call it recursively.
	pub fn appendix() -> StoragePagedListMeta<Prefix, Value, Hasher, ValuesPerPage> {
		Self::read_meta()
	}

	// convenience stuff for tests
	#[cfg(feature = "std")]
	pub fn as_vec() -> Vec<Value> {
		<Self as crate::storage::StoragePagedList<_>>::iter().collect()
	}

	// convenience stuff for tests
	#[cfg(feature = "std")]
	pub fn drain_as_vec() -> Vec<Value> {
		<Self as crate::storage::StoragePagedList<_>>::drain().collect()
	}
}

/// Prelude for doc-tests.
#[cfg(feature = "std")]
#[allow(dead_code)]
pub(crate) mod test_predlude {
	pub use super::*;
	pub use crate::{
		hash::*,
		metadata_ir::{StorageEntryModifierIR, StorageEntryTypeIR, StorageHasherIR},
		parameter_types,
		storage::{types::ValueQuery, StoragePagedList as _},
	};
	pub use sp_io::{hashing::twox_128, TestExternalities};

	parameter_types! {
		pub const ValuesPerPage: u32 = 5;
		pub const MaxPages: Option<u32> = Some(20);
	}

	pub struct Prefix;
	impl StorageInstance for Prefix {
		fn pallet_prefix() -> &'static str {
			"test"
		}
		const STORAGE_PREFIX: &'static str = "foo";
	}

	pub type List = StoragePagedList<Prefix, Blake2_128Concat, u32, ValuesPerPage, MaxPages>;
}

#[cfg(test)]
mod tests {
	use super::test_predlude::*;

	#[test]
	fn append_works() {
		TestExternalities::default().execute_with(|| {
			List::append_many(0..1000);
			assert_eq!(List::as_vec(), (0..1000).collect::<Vec<_>>());
		});
	}

	/// Draining all works.
	#[test]
	fn simple_drain_works() {
		TestExternalities::default().execute_with(|| {
			let _g = StorageNoopGuard::default(); // All in all a No-Op
			List::append_many(0..1000);

			assert_eq!(List::drain_as_vec(), (0..1000).collect::<Vec<_>>());

			assert_eq!(List::read_meta(), Default::default());

			// all gone
			assert_eq!(List::as_vec(), Vec::<u32>::new());
			// Need to delete the metadata manually.
			StoragePagedListMeta::<Prefix, u32, Blake2_128Concat, ValuesPerPage>::clear();
		});
	}

	/// Drain half of the elements and iterator the rest.
	#[test]
	fn partial_drain_works() {
		TestExternalities::default().execute_with(|| {
			List::append_many(0..100);

			let vals = List::drain().take(50).collect::<Vec<_>>();
			assert_eq!(vals, (0..50).collect::<Vec<_>>());

			let meta = List::read_meta();
			// Will switch over to `10/0`, but will in the next call.
			assert_eq!((meta.first_page, meta.first_value), (9, 5));

			// 50 gone, 50 to go
			assert_eq!(List::as_vec(), (50..100).collect::<Vec<_>>());
		});
	}

	/// Draining, appending and iterating work together.
	#[test]
	fn drain_append_iter_works() {
		sp_tracing::try_init_simple();

		TestExternalities::default().execute_with(|| {
			for r in 1..=100 {
				List::append_many(0..12);
				List::append_many(0..12);

				let dropped = List::drain().take(12).collect::<Vec<_>>();
				assert_eq!(dropped, (0..12).collect::<Vec<_>>());

				assert_eq!(List::as_vec(), (0..12).cycle().take(r * 12).collect::<Vec<_>>());
			}
		});
	}

	#[test]
	fn peekable_drain_also_deletes() {
		sp_tracing::try_init_simple();

		TestExternalities::default().execute_with(|| {
			List::append_many(0..10);

			let mut iter = List::drain().peekable();
			assert_eq!(iter.peek(), Some(&0));
			// `peek` does remove one element...
			assert_eq!(List::iter().count(), 9);
		});
	}
}
