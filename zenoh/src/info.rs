//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

//! Tools to access information about the current zenoh [`Session`](crate::Session).
use crate::SessionRef;
use std::future::{IntoFuture, Ready};
use zenoh_config::{WhatAmI, ZenohId};
use zenoh_core::{IntoFutureSend, Resolvable, Resolve};

/// A builder retuned by [`SessionInfo::zid()`](SessionInfo::zid) that allows
/// to access the [`ZenohId`] of the current zenoh [`Session`](crate::Session).
///
/// # Examples
/// ```
/// # async_std::task::block_on(async {
/// use zenoh::prelude::*;
///
/// let session = zenoh::open(config::peer()).await.unwrap();
/// let zid = session.info().zid().await;
/// # })
/// ```
pub struct ZidBuilder<'a> {
    pub(crate) session: SessionRef<'a>,
}

impl<'a> Resolvable for ZidBuilder<'a> {
    type To = ZenohId;
}

impl<'a> Resolve<<Self as Resolvable>::To> for ZidBuilder<'a> {
    fn wait(self) -> Self::To {
        self.session.runtime.zid
    }
}

impl<'a> IntoFutureSend for ZidBuilder<'a> {
    type Future = Ready<Self::To>;

    fn into_future_send(self) -> Self::Future {
        std::future::ready(self.wait())
    }
}

impl<'a> IntoFuture for ZidBuilder<'a> {
    type Output = <Self as Resolvable>::To;
    type IntoFuture = <Self as IntoFutureSend>::Future;

    fn into_future(self) -> Self::IntoFuture {
        self.into_future_send()
    }
}

/// A builder returned by [`SessionInfo::routers_zid()`](SessionInfo::routers_zid) that allows
/// to access the [`ZenohId`] of the zenoh routers this process is currently connected to
/// or the [`ZenohId`] of the current router if this code is run from a router (plugin).
///
/// # Examples
/// ```
/// # async_std::task::block_on(async {
/// use zenoh::prelude::*;
///
/// let session = zenoh::open(config::peer()).await.unwrap();
/// let mut routers_zid = session.info().routers_zid().await;
/// while let Some(router_zid) = routers_zid.next() {}
/// # })
/// ```
pub struct RoutersZidBuilder<'a> {
    pub(crate) session: SessionRef<'a>,
}

impl<'a> Resolvable for RoutersZidBuilder<'a> {
    type To = Box<dyn Iterator<Item = ZenohId> + Send + Sync>;
}

impl<'a> Resolve<<Self as Resolvable>::To> for RoutersZidBuilder<'a> {
    fn wait(self) -> Self::To {
        Box::new(
            self.session
                .runtime
                .manager()
                .get_transports()
                .into_iter()
                .filter_map(|s| {
                    s.get_whatami()
                        .ok()
                        .and_then(|what| (what == WhatAmI::Router).then_some(()))
                        .and_then(|_| s.get_zid().ok())
                }),
        )
    }
}

impl<'a> IntoFutureSend for RoutersZidBuilder<'a> {
    type Future = Ready<Self::To>;

    fn into_future_send(self) -> Self::Future {
        std::future::ready(self.wait())
    }
}

impl<'a> IntoFuture for RoutersZidBuilder<'a> {
    type Output = <Self as Resolvable>::To;
    type IntoFuture = <Self as IntoFutureSend>::Future;

    fn into_future(self) -> Self::IntoFuture {
        self.into_future_send()
    }
}

/// A builder retuned by [`SessionInfo::peers_zid()`](SessionInfo::peers_zid) that allows
/// to access the [`ZenohId`] of the zenoh peers this process is currently connected to.
///
/// # Examples
/// ```
/// # async_std::task::block_on(async {
/// use zenoh::prelude::*;
///
/// let session = zenoh::open(config::peer()).await.unwrap();
/// let zid = session.info().zid().await;
/// let mut peers_zid = session.info().peers_zid().await;
/// while let Some(peer_zid) = peers_zid.next() {}
/// # })
/// ```
pub struct PeersZidBuilder<'a> {
    pub(crate) session: SessionRef<'a>,
}

impl<'a> Resolvable for PeersZidBuilder<'a> {
    type To = Box<dyn Iterator<Item = ZenohId> + Send + Sync>;
}

impl<'a> Resolve<<Self as Resolvable>::To> for PeersZidBuilder<'a> {
    fn wait(self) -> <Self as Resolvable>::To {
        Box::new(
            self.session
                .runtime
                .manager()
                .get_transports()
                .into_iter()
                .filter_map(|s| {
                    s.get_whatami()
                        .ok()
                        .and_then(|what| (what == WhatAmI::Peer).then_some(()))
                        .and_then(|_| s.get_zid().ok())
                }),
        )
    }
}

impl<'a> IntoFutureSend for PeersZidBuilder<'a> {
    type Future = Ready<Self::To>;

    fn into_future_send(self) -> Self::Future {
        std::future::ready(self.wait())
    }
}

impl<'a> IntoFuture for PeersZidBuilder<'a> {
    type Output = <Self as Resolvable>::To;
    type IntoFuture = std::future::Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        self.into_future_send()
    }
}

/// Struct returned by [`Session::info()`](crate::Session::info) which allows
/// to access informations about the current zenoh [`Session`](crate::Session).
///
/// # Examples
/// ```
/// # async_std::task::block_on(async {
/// use zenoh::prelude::*;
///
/// let session = zenoh::open(config::peer()).await.unwrap();
/// let info = session.info();
/// let zid = info.zid().await;
/// # })
/// ```
pub struct SessionInfo<'a> {
    pub(crate) session: SessionRef<'a>,
}

impl SessionInfo<'_> {
    /// Return the [`ZenohId`] of the current zenoh [`Session`](crate::Session).
    ///
    /// # Examples
    /// ```
    /// # async_std::task::block_on(async {
    /// use zenoh::prelude::*;
    ///
    /// let session = zenoh::open(config::peer()).await.unwrap();
    /// let zid = session.info().zid().await;
    /// # })
    /// ```
    pub fn zid(&self) -> ZidBuilder<'_> {
        ZidBuilder {
            session: self.session.clone(),
        }
    }

    /// Return the [`ZenohId`] of the zenoh routers this process is currently connected to
    /// or the [`ZenohId`] of the current router if this code is run from a router (plugin).
    ///
    /// # Examples
    /// ```
    /// # async_std::task::block_on(async {
    /// use zenoh::prelude::*;
    ///
    /// let session = zenoh::open(config::peer()).await.unwrap();
    /// let mut routers_zid = session.info().routers_zid().await;
    /// while let Some(router_zid) = routers_zid.next() {}
    /// # })
    /// ```
    pub fn routers_zid(&self) -> RoutersZidBuilder<'_> {
        RoutersZidBuilder {
            session: self.session.clone(),
        }
    }

    /// Return the [`ZenohId`] of the zenoh peers this process is currently connected to.
    ///
    /// # Examples
    /// ```
    /// # async_std::task::block_on(async {
    /// use zenoh::prelude::*;
    ///
    /// let session = zenoh::open(config::peer()).await.unwrap();
    /// let mut peers_zid = session.info().peers_zid().await;
    /// while let Some(peer_zid) = peers_zid.next() {}
    /// # })
    /// ```
    pub fn peers_zid(&self) -> PeersZidBuilder<'_> {
        PeersZidBuilder {
            session: self.session.clone(),
        }
    }
}
