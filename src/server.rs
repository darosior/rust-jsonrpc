// Rust JSON-RPC Library
// Written in 2015 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Server support
//!
//! Support for listening for JSONRPC requests over miscellaneous transports, and
//! sending back JSONRPC responses (or errors!)
//!

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic;

use serde;
use serde_json;
use serde_json::value::RawValue;

use super::{Request, Response};
use error::Error;
use util::HashableValue;

/// An interface for a transport over which to use the JSONRPC protocol.
pub trait ServerTransport: Send + Sync + 'static {
    /// Send an RPC response over the transport.
    fn send_response(&self, Response) -> Result<Response, Error>;
    /// Send a batch of RPC responses over the transport.
    fn send_batch(&self, &[Response]) -> Result<Vec<Response>, Error>;
    /// Format the target of this transport.
    /// I.e. the URL/socket/...
    fn fmt_target(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

/// A JSON-RPC sert.
///
/// Create a new Server using one of the transport-specific constructors.
pub struct Server {
    pub(crate) transport: Box<dyn ServerTransport>,
    nonce: atomic::AtomicUsize,
    // TODO: we need an optional context structure to optionally pass to callbacks
}

impl Server {
    /// Creates a new client with the given transport.
    pub fn with_transport<T: ServerTransport>(transport: T) -> Server {
        Builder::with_transport(transport).build()
    }
}

impl fmt::Debug for ::Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "jsonrpc::Server(")?;
        self.transport.fmt_target(f)?;
        write!(f, ")")
    }
}

pub struct Builder {
    server: Server
}

impl Builder {
    /// Creates a new client with the given transport.
    pub fn with_transport<T: ServerTransport>(transport: T) -> Builder {
        Builder {
            server: Server {
                transport: Box::new(transport),
                nonce: atomic::AtomicUsize::new(0),
            }
        }
    }

    pub fn build() -> Server {
        self.server
    }
}
