// Copyright 2017-2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-archive.

// substrate-archive is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// substrate-archive is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with substrate-archive.  If not, see <http://www.gnu.org/licenses/>.

//! Wrapper RPC convenience functions

use desub::decoder::Metadata;
use futures::{future::join, TryFutureExt};
use runtime_version::RuntimeVersion;
use sp_runtime::traits::{Block as _, Header as HeaderTrait};
use substrate_rpc_primitives::number::NumberOrHex;
use subxt::Client;

use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    error::Error as ArchiveError,
    types::{BatchData, Data, Header, Storage, Substrate, SubstrateBlock},
};

/// Communicate with Substrate node via RPC
#[derive(Clone)]
pub struct Rpc<T: Substrate + Send + Sync> {
    client: Client<T>,
}

/// Methods that return fetched value directly
impl<T> Rpc<T>
where
    T: Substrate + Send + Sync,
{
    pub fn new(client: subxt::Client<T>) -> Self {
        Self { client }
    }

    /// Fetch a block by hash from Substrate RPC
    pub(crate) async fn block(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<Option<SubstrateBlock<T>>, ArchiveError> {
        self.client.block(hash).await.map_err(Into::into)
    }

    pub(crate) async fn meta_and_version(
        &self,
        hash: Option<T::Hash>,
    ) -> Result<(RuntimeVersion, Metadata), ArchiveError> {
        let meta = self
            .client
            .raw_metadata(hash.as_ref())
            .map_err(ArchiveError::from);
        let version = self
            .client
            .runtime_version(hash.as_ref())
            .map_err(ArchiveError::from);
        let (meta, version) = join(meta, version).await;
        let meta = meta?;
        let version = version?;
        Ok((version, Metadata::new(meta.as_slice())))
    }

    pub async fn block_from_number(
        &self,
        number: NumberOrHex<T::BlockNumber>,
    ) -> Result<Option<SubstrateBlock<T>>, ArchiveError> {
        let hash = self.client.block_hash(Some(number.into())).await?;
        self.client.block(hash).await.map_err(Into::into)
    }

    // pub async fn storage()

    /// unsubscribe from finalized heads
    #[allow(dead_code)]
    fn unsubscribe_finalized_heads() {
        unimplemented!();
    }

    /// unsubscribe from new heads
    #[allow(dead_code)]
    fn unsubscribe_new_heads() {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_query_blocks() {}
}
