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

mod blocks;
mod database;
mod metadata;
mod state_tracing;
mod storage_aggregator;

/// Database message to get state internal database state
pub use self::database::{DatabaseActor, GetState};
pub use self::metadata::Metadata;
pub use blocks::BlocksIndexer;
pub use state_tracing::{Traces, TracingActor};
pub use storage_aggregator::StorageAggregator;

use super::actor_pool::ActorPool;
use super::msg::Die;