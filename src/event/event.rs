use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};

use super::RawEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    Course,
    Exam,
    Leave,
    Meeting,
    PracticalWork,
    SupervisedWork,
    Project,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    id: u32,
    kind: EventKind,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    rooms: Vec<String>,
    subject: String,
    chapter: Option<String>,
    participants: Vec<String>,
}

impl Event {
    /// Parse a raw event into an event.
    pub fn from_raw_event(event: RawEvent) -> Result<Event, Box<dyn std::error::Error>> {
        parse_event(event)
    }
}

fn map_kind<T: Into<String>>(event_type: T) -> EventKind {
    match event_type.into().to_lowercase().as_str() {
        "conges" => EventKind::Leave,
        "cm" => EventKind::Course,
        "cours" => EventKind::Course,
        "est-epreuve" => EventKind::Exam,
        "evaluation" => EventKind::Exam,
        "ds" => EventKind::Exam,
        "reunion" => EventKind::Meeting,
        "td" => EventKind::SupervisedWork,
        "cours_td" => EventKind::SupervisedWork,
        "tp" => EventKind::PracticalWork,
        "projet" => EventKind::Project,
        _ => EventKind::Other,
    }
}

/// Parse a raw event into an event.
fn parse_event(event: RawEvent) -> Result<Event, Box<dyn std::error::Error>> {
    let id: u32 = event.id.parse().unwrap();
    let kind = map_kind(event.className);

    // Parse the raw title into the room, subject, chapter and participants
    let result = parse_title(event.title);
    let (rooms, subject, chapter, participants) = match result {
        Ok((rooms, subject, chapter, participants)) => (rooms, subject, chapter, participants),
        Err(e) => {
            error!("Failed to parse event title: {}", e);
            return Err(e);
        }
    };

    Ok(Event {
        id,
        kind,
        start: event.start,
        end: event.end,
        rooms,
        subject,
        chapter,
        participants,
    })
}

/// Parse the title of an event into the room, subject, chapter and participants.
/// The title is of the form "12h00 à 13h00 - ...".
fn parse_title<T: Into<String>>(
    title: T,
) -> Result<(Vec<String>, String, Option<String>, Vec<String>), Box<dyn std::error::Error>> {
    let title = title.into();
    let mut rooms: Vec<String> = Vec::new();
    let subject: String;
    let mut chapter: Option<String> = None;
    let mut participants: Vec<String> = Vec::new();

    // Check if whether the title is of the form "12h00 à 13h00 - ..." or
    // "12h00 - 13h00 - ...". The first case is used by ISEN Ouest, the second
    // by ISEN Lille.
    if title.chars().nth(6) == Some('à') {
        // The chapter can contain a separator " - ", so we need to be careful when
        // splitting the title.

        // Clean the title by removing the first 16 characters.
        // And then split the title by the end (the last " - " separator)
        let title = title[16..].rsplit_once(" - ").unwrap().0;
        let title = title.split(" - ").collect::<Vec<&str>>();

        // The first element is the rooms
        for room in title[0].split(" / ") {
            let room = room.trim();
            rooms.push(room.to_string());
        }

        // The third element is the subject
        subject = title[2].to_string();

        // The fourth to n - 2 elements is the chapter
        let _chapter = title[3..title.len() - 1].join(" - ");
        let _chapter = _chapter.trim();
        if !_chapter.is_empty() {
            chapter = Some(_chapter.to_string());
        }

        // The last element is the participants
        for participant in title[title.len() - 1].split(" / ") {
            let participant = participant.trim();
            if !participant.is_empty() {
                participants.push(participant.to_string());
            }
        }
    } else if title.chars().nth(6) == Some('-') {
        // TODO: Implement the second case (ISEN Lille)
        panic!("Not implemented yet");
    } else {
        error!("The title is not of the form \"12h00 à 13h00 - ...\" or \"12h00 - 13h00 - ...\".");
        return Err(
            "The title is not of the form \"12h00 à 13h00 - ...\" or \"12h00 - 13h00 - ...\"."
                .into(),
        );
    }

    Ok((rooms, subject, chapter, participants))
}
