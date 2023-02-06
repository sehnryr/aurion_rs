#![deny(missing_docs)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// RawEvent is the raw event data that is sent to the client.
/// It is used to create the Event struct.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEvent {
    /// The id of the event.
    pub id: String,

    /// The title of the event.
    /// The title is of the form "12h00 à 13h00 - ...".
    /// The title is parsed into the room, subject, chapter and participants.
    pub title: String,

    /// The start date and time of the event.
    pub start: DateTime<Utc>,

    /// The end date and time of the event.
    pub end: DateTime<Utc>,

    // The following boolean fields are not used by the client.
    allDay: bool,
    editable: bool,

    /// The class name of the event.
    /// The class name is used to determine the kind of the event.
    /// The class name is parsed into the kind of the event.
    pub className: String,
}
