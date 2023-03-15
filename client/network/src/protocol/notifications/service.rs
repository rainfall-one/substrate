// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Notification service implementation.

#![allow(unused)]

use crate::{
	protocol::notifications::handler::NotificationsSink,
	service::traits::{NotificationEvent, NotificationService, ValidationResult},
	types::ProtocolName,
};

use futures::{
	// channel::{mpsc, oneshot},
	stream::Stream,
	SinkExt,
	StreamExt,
};
use libp2p::PeerId;
use tokio::sync::{mpsc, oneshot};

use sc_network_common::role::ObservedRole;

use std::{collections::HashMap, fmt::Debug};

// TODO: documentation
enum InnerNotificationEvent {
	/// Validate inbound substream.
	ValidateInboundSubstream {
		/// Peer ID.
		peer: PeerId,

		/// Received handshake.
		handshake: Vec<u8>,

		/// `oneshot::Sender` for sending validation result back to `Notifications`
		result_tx: oneshot::Sender<ValidationResult>,
	},

	/// Remote identified by `PeerId` opened a substream and sent `Handshake`.
	/// Validate `Handshake` and report status (accept/reject) to `Notifications`.
	NotificationStreamOpened {
		/// Peer ID.
		peer: PeerId,

		/// Role of the peer.
		role: ObservedRole,

		/// Negotiated fallback.
		negotiated_fallback: Option<ProtocolName>,

		/// Notification sink.
		sink: NotificationsSink,
	},

	/// Substream was closed.
	NotificationStreamClosed {
		/// Peer Id.
		peer: PeerId,
	},

	/// Notification was received from the substream.
	NotificationReceived {
		/// Peer ID.
		peer: PeerId,

		/// Received notification.
		notification: Vec<u8>,
	},
}

/// Notification commands.
///
/// Commands sent by the notifications protocol to `Notifications`
/// in order to modify connectivity state and communicate with the remote peer.
pub enum NotificationCommand {
	/// Instruct `Notifications` to open a substream to peer.
	OpenSubstream(PeerId),

	/// Instruct `Notifications` to close the substream to peer.
	CloseSubstream(PeerId),

	/// Send notification to peer.
	SendNotification(PeerId, Vec<u8>),

	/// Set handshake for the notifications protocol.
	SetHandshake(Vec<u8>),
}

/// Handle that is passed on to the notifications protocol.
#[derive(Debug)]
pub struct NotificationHandle {
	/// TX channel for sending commands to `Notifications`.
	tx: mpsc::Sender<NotificationCommand>,

	/// RX channel for receiving events from `Notifications`.
	rx: mpsc::Receiver<InnerNotificationEvent>,

	/// Connected peers.
	peers: HashMap<PeerId, NotificationsSink>,
}

impl NotificationHandle {
	/// Create new [`NotificationHandle`].
	fn new(
		tx: mpsc::Sender<NotificationCommand>,
		rx: mpsc::Receiver<InnerNotificationEvent>,
	) -> Self {
		Self { tx, rx, peers: HashMap::new() }
	}
}

#[async_trait::async_trait]
impl NotificationService for NotificationHandle {
	/// Instruct `Notifications` to open a new substream for `peer`.
	async fn open_substream(&mut self, peer: PeerId) -> Result<(), ()> {
		todo!()
	}

	/// Instruct `Notifications` to close substream for `peer`.
	async fn close_substream(&mut self, peer: PeerId) -> Result<(), ()> {
		todo!();
	}

	/// Send synchronous `notification` to `peer`.
	fn send_sync_notification(&mut self, peer: PeerId, notification: Vec<u8>) -> Result<(), ()> {
		todo!();
	}

	/// Send asynchronous `notification` to `peer`, allowing sender to exercise backpressure.
	async fn send_async_notification(
		&mut self,
		peer: PeerId,
		notification: Vec<u8>,
	) -> Result<(), ()> {
		todo!();
	}

	/// Set handshake for the notification protocol replacing the old handshake.
	async fn set_hanshake(&mut self, handshake: Vec<u8>) -> Result<(), ()> {
		self.tx.send(NotificationCommand::SetHandshake(handshake)).await.map_err(|_| ())
	}

	/// Get next event from the `Notifications` event stream.
	async fn next_event(&mut self) -> Option<NotificationEvent> {
		todo!();
		// match self.rx.next().await {}
	}
}

/// Handle that is passed on to `Notifications` and allows it to directly communicate
/// with the protocol.
#[derive(Debug)]
pub struct ProtocolHandle {
	/// TX channel for sending events to protocol.
	tx: mpsc::Sender<InnerNotificationEvent>,

	/// RX channel for receiving commands from `Protocol`.
	rx: mpsc::Receiver<NotificationCommand>,
}

pub trait ProtocolService: Debug {
	/// Report to the protocol that a substream has been opened and it must be validated by the
	/// protocol.
	///
	/// Return `oneshot::Receiver` which allows `Notifications` to poll for the validation result
	/// from protocol.
	fn report_incoming_substream(
		&mut self,
		peer: PeerId,
		handshake: Vec<u8>,
	) -> Result<oneshot::Receiver<ValidationResult>, ()>;

	fn report_substream_opened(
		&mut self,
		peer: PeerId,
		role: ObservedRole,
		negotiated_fallback: Option<ProtocolName>,
		sink: NotificationsSink,
	) -> Result<(), ()>;

	/// Substream was closed.
	fn report_substream_closed(&mut self, peer: PeerId) -> Result<(), ()>;

	/// Notification was received from the substream.
	fn report_notification_received(
		&mut self,
		peer: PeerId,
		notification: Vec<u8>,
	) -> Result<(), ()>;
}

impl ProtocolService for mpsc::Sender<InnerNotificationEvent> {
	/// Report to the protocol that a substream has been opened and it must be validated by the
	/// protocol.
	///
	/// Return `oneshot::Receiver` which allows `Notifications` to poll for the validation result
	/// from protocol.
	fn report_incoming_substream(
		&mut self,
		peer: PeerId,
		handshake: Vec<u8>,
	) -> Result<oneshot::Receiver<ValidationResult>, ()> {
		todo!();
	}

	fn report_substream_opened(
		&mut self,
		peer: PeerId,
		role: ObservedRole,
		negotiated_fallback: Option<ProtocolName>,
		sink: NotificationsSink,
	) -> Result<(), ()> {
		todo!()
	}

	/// Substream was closed.
	fn report_substream_closed(&mut self, peer: PeerId) -> Result<(), ()> {
		todo!()
	}

	/// Notification was received from the substream.
	fn report_notification_received(
		&mut self,
		peer: PeerId,
		notification: Vec<u8>,
	) -> Result<(), ()> {
		todo!()
	}
}

// TODO: the other object doesn't have to be boxed, can be normal struct
impl ProtocolHandle {
	/// Create new [`ProtocolHandle`].
	fn new(
		tx: mpsc::Sender<InnerNotificationEvent>,
		rx: mpsc::Receiver<NotificationCommand>,
	) -> Self {
		Self { tx, rx }
	}

	/// Split [`ProtocolHandle`] into an object implementing trait that allows `Notifications`
	/// to interact with the protocol and into a stream of commands received from the protocol.
	pub fn split(self) -> (Box<dyn ProtocolService>, Box<dyn Stream<Item = NotificationCommand>>) {
		(Box::new(self.tx), Box::new(tokio_stream::wrappers::ReceiverStream::new(self.rx)))
	}
}

/// Create new (protocol, notification) handle pair.
///
/// Handle pair allows `Notifications` and the protocol to communicate with each other directly.
pub fn notification_service() -> (ProtocolHandle, Box<dyn NotificationService>) {
	let (cmd_tx, cmd_rx) = mpsc::channel(64); // TODO: zzz
	let (event_tx, event_rx) = mpsc::channel(64); // TODO: zzz

	(ProtocolHandle::new(event_tx, cmd_rx), Box::new(NotificationHandle::new(cmd_tx, event_rx)))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn validate_and_accept_substream() {
		let (proto, notif) = notification_service();
		let (sink, _, _) = NotificationsSink::new(PeerId::random());

		let (tx, rx) = proto.split();

		// TODO: emit
	}

	#[tokio::test]
	async fn validate_and_reject_substream() {
		let (proto, notif) = notification_service();
		let (sink, _, _) = NotificationsSink::new(PeerId::random());

		let (tx, rx) = proto.split();

		// TODO: emit
	}
}
