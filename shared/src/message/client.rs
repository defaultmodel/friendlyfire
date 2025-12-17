use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{DisplayOptions, Overlay, message::version::Version};

/// Top-level message sent by a client to the server.
///
/// Client messages are *requests* or *signals* only.
/// As only `ServerMessage` is autoritative.
///
/// Contrary to `ServerMessage` no `sender_info` is included on purpose:
/// the server never trusts client-provided data, espcially identity.
///
/// NOTE : No `ClientMessage` should ever be received by any client as they always go through
/// the server and are converted into `ServerMessage`
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    /// Protocol version of the message payload.
    /// Used by the server to validate compatibility.
    pub version: Version,

    /// Actual message payload.
    #[serde(flatten)]
    pub kind: ClientMessageType,
}

/// All possible client message kinds.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessageType {
    /// Request creation of a new party.
    /// The sender of this `ClientMessageType` will have the `Role::Creator`.
    CreateParty,

    /// Request disbanding of a party.
    /// Permission checks (e.g. admin rights) are enforced server-side.
    DisbandParty { party_id: Uuid },

    /// Request to join a party using an invitation link.
    JoinParty { invitation_link: String },

    /// Request generation of a new invitation link for a given party.
    /// Restricted to `User` with `role` of `Role::Admin` or `Role::Creator`.
    CreateInvationLink { party_id: Uuid },

    /// Broadcast a set of overlays through the server.
    Overlays {
        overlays: Vec<Overlay>,
        options: DisplayOptions,
    },

    /// Acknowledge successful download of all overlays.
    /// See `ServerMessageType::OverlaysFullAck`, to see it's use.
    OverlaysAck,

    /// Acknowledge successful rasterization of all overlays.
    /// See `ServerMessageType::RasterizationFullAck`, to see it's use.
    RasterizationAck,

    /// Signal readiness to trigger the final action.
    /// The server decides if and when this becomes authoritative.
    Fire,

    /// Error emitted by the client.
    Error { message: String },
}
