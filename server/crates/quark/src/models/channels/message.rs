use crate::util::regex::RE_COLOUR;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use validator::Validate;

use iso8601_timestamp::Timestamp;

#[cfg(feature = "rocket_impl")]
use rocket::FromFormField;

use crate::{
    models::{attachment::File, Member, User},
    types::january::Embed,
};

/// Utility function to check if a boolean value is false
pub fn if_false(t: &bool) -> bool {
    !t
}

/// # Reply
///
/// Representation of a message reply before it is sent.
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Reply {
    /// Message Id
    pub id: String,
    /// Whether this reply should mention the message's author
    pub mention: bool,
}

/// Representation of a text embed before it is sent.
#[derive(Validate, Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct SendableEmbed {
    #[validate(length(min = 1, max = 128))]
    pub icon_url: Option<String>,
    pub url: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 2000))]
    pub description: Option<String>,
    pub media: Option<String>,
    #[validate(length(min = 1, max = 128), regex = "RE_COLOUR")]
    pub colour: Option<String>,
}

/// Representation of a system event message
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "type")]
pub enum SystemMessage {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "user_added")]
    UserAdded { id: String, by: String },
    #[serde(rename = "user_remove")]
    UserRemove { id: String, by: String },
    #[serde(rename = "user_joined")]
    UserJoined { id: String },
    #[serde(rename = "user_left")]
    UserLeft { id: String },
    #[serde(rename = "user_kicked")]
    UserKicked { id: String },
    #[serde(rename = "user_banned")]
    UserBanned { id: String },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { name: String, by: String },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged { by: String },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged { by: String },
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged { from: String, to: String },
}

/// Name and / or avatar override information
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Validate)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 32))]
    pub name: Option<String>,
    /// Replace the avatar shown on this message (URL to image file)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 128))]
    pub avatar: Option<String>,
    /// Replace the display role colour shown on this message
    ///
    /// Must have `ManageRole` permission to use
    ///
    /// This can be any valid CSS colour
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 32))]
    pub colour: Option<String>,
}

/// Information to guide interactions on this message
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Validate, Default)]
pub struct Interactions {
    /// Reactions which should always appear and be distinct
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reactions: Option<HashSet<String>>,
    /// Whether reactions should be restricted to the given list
    #[serde(skip_serializing_if = "if_false", default)]
    pub restrict_reactions: bool,
}

/// Representation of a Message on Revolt
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, OptionalStruct, Default)]
#[optional_derive(Serialize, Deserialize, JsonSchema, Debug, Default, Clone)]
#[optional_name = "PartialMessage"]
#[opt_skip_serializing_none]
#[opt_some_priority]
pub struct Message {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Unique value generated by client sending this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// Id of the channel this message was sent in
    pub channel: String,
    /// Id of the user that sent this message
    pub author: String,

    /// Message content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// System message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemMessage>,
    /// Array of attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<File>>,
    /// Time at which this message was last edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited: Option<Timestamp>,
    /// Attached embeds to this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    /// Array of user ids mentioned in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<String>>,
    /// Array of message ids this message is replying to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<String>>,
    /// Hashmap of emoji IDs to array of user IDs
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub reactions: HashMap<String, HashSet<String>>,
    /// Information about how this message should be interacted with
    #[serde(skip_serializing_if = "Interactions::is_default", default)]
    pub interactions: Interactions,
    /// Name and / or avatar overrides for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Masquerade>,
}

/// # Message Sort
///
/// Sort used for retrieving messages
#[derive(Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "rocket_impl", derive(FromFormField))]
pub enum MessageSort {
    /// Sort by the most relevant messages
    Relevance,
    /// Sort by the newest messages first
    Latest,
    /// Sort by the oldest messages first
    Oldest,
}

impl Default for MessageSort {
    fn default() -> MessageSort {
        MessageSort::Relevance
    }
}

/// # Bulk Message Response
///
/// Response used when multiple messages are fetched
#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub enum BulkMessageResponse {
    JustMessages(
        /// List of messages
        Vec<Message>,
    ),
    MessagesAndUsers {
        /// List of messages
        messages: Vec<Message>,
        /// List of users
        users: Vec<User>,
        /// List of members
        #[serde(skip_serializing_if = "Option::is_none")]
        members: Option<Vec<Member>>,
    },
}

/// # Appended Information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppendMessage {
    /// Additional embeds to include in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
}
