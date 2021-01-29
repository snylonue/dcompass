// Copyright 2020 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::client_pool::ClientPoolError;
use crate::Label;
use std::{collections::HashSet, fmt::Debug};
use thiserror::Error;
use tokio::time::error::Elapsed;
use trust_dns_client::error::ClientError;
use trust_dns_proto::error::ProtoError;

pub type Result<T> = std::result::Result<T, UpstreamError>;

/// Error generated by the `upstreams` section.
#[derive(Error, Debug)]
pub enum UpstreamError {
    /// Tag missing in upstream definition for either the destination of a rule or the `default_tag`
    #[error("No upstream with tag `{0}` found")]
    MissingTag(Label),

    /// There are multiple definitions of rules of the same destination or upstreams of the same tag name.
    #[error("Multiple defintions found for tag `{0}` in the `upstreams`")]
    MultipleDef(Label),

    /// Hybrid definition forms a chain, which is prohibited
    #[error("You cannot recursively define `hybrid` method. The `hybrid` method that contains the destination to be recursively called: {0}")]
    HybridRecursion(Label),

    /// There is no destinations in hybrid's destination list.
    #[error("`hybrid` upstream method with tag `{0}` contains no upstreams to race")]
    EmptyHybrid(Label),

    /// Error forwarded from `trust-dns-client`.
    #[error(transparent)]
    ClientError(#[from] ClientError),

    /// Error originated from client pools.
    #[error(transparent)]
    ClientPoolError(#[from] ClientPoolError),

    /// Error forwarded from `trust-dns-proto`.
    #[error(transparent)]
    ProtoError(#[from] ProtoError),

    /// Error forwarded from `tokio::time::error`. This indicates a timeout probably.
    #[error(transparent)]
    TimeError(#[from] Elapsed),

    /// Some of the upstreams are unused.
    #[error("Some of the upstreams are not used: {0:?}")]
    UnusedUpstreams(HashSet<Label>),
}
