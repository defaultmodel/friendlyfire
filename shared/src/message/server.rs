use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{DisplayOptions, Overlay, message::version::Version};

#[derive(Serialize, Deserialize, Debug)]
/// Top-level message emitted by the server
/// Server messages are authoritative and should never be rejected or altered by clients
pub struct ServerMessage {
    /// Friendlyfire protocol version of the server used to send the message.
    /// Used to detect incompatibilities between client and server.
    pub version: Version,
    pub sender: SenderInfo,

    // Flattened to avoid a "kind" object in the message that isn't really useful.
    /// Actual message payload.
    #[serde(flatten)]
    pub kind: ServerMessageType,
}

// TODO : Could be expanded a subset of `User` attributes.
// `let senderInfo = User.into()` should be possible
#[derive(Serialize, Deserialize, Debug)]
pub struct SenderInfo {
    /// Stable server-assigned identifier of the sender.
    // TODO : Link this to `UserId` ?
    pub id: Uuid,
}

/// All possible server message kinds.
///
/// When a `ClientMessageType` is received on the server it is always converted into a `ServerMessage` of the same enum value.
/// e.g. : `ClientMessageType::Fire` becomes when he is relayed through the server `ServerMessageType::Fire`
/// This garanties the message to be authoritative and right (no foul play by the client)
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessageType {
    /// Response to `ClientMessageType::CreateParty`
    /// Confirms party creation and assigns creator(same as admin) privileges to the requesting client.
    PartyCreated,

    /// Response to `ClientMessageType::JoinParty`
    /// This tells the client he successfully joined the given `Party`
    JoinAccepted,

    /// Relay of the `ClientMessageType::Overlays`
    /// Contains the full set of overlays to be displayed along with metadata in `options` to adjust the displaying.
    Overlays {
        overlays: Vec<Overlay>,
        options: DisplayOptions,
    },

    /// Aggregate of `ClientMessageType::OverlaysAck`
    /// Sent when all online members of a party have downloaded the `Overlays`
    OverlaysFullAck,

    /// Aggregate of `ClientMessageType::RasterizationAck`
    /// Sent when all online members of a party have rasterized all the `Overlays`
    RasterizationFullAck,

    /// Relay of the `ClientMessageType::Fire`
    /// Can only be sent once `OverlaysFullAck` and `RasterizationFullAck` have been emitted
    Fire,

    /// Error emitted by the server.
    /// Indicates a rejected client action or a server error.
    Error { message: String },
}
