use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use crate::models::{enums::audiovisual::RecordingType, root_structs::media::{HasMedia, Media}, traits::NonRootStruct};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audio {
    media:              Media,

    // File Metadata
    duration_seconds:   Option<i32>,
    recording_type:     RecordingType,
    sample_rate_hz:     Option<i32>,
    bit_depth:          Option<i16>,
    bitrate_kbps:       Option<i32>,
    channel_layout:     Option<String>,
    is_lossless:        bool,
    loudness_lufs:      Option<Decimal>,
    dynamic_range_db:   Option<Decimal>,

    // Content
    is_dialogue_driven: bool,
    has_transcript:     bool,
    spoken_language_id: Option<i32>,
    recorded_on:        Option<NaiveDate>,
    recording_venue:    Option<String>
}

impl HasMedia for Audio {
    fn media(&self) -> &Media {
        &self.media
    }
}

pub trait HasAudio: HasMedia {
    fn audio(&self) -> &Audio;

    // File Metadata
    fn duration_seconds(&self) -> Option<&i32> { self.audio().duration_seconds() }
    fn recording_type(&self) -> &RecordingType { self.audio().recording_type() }
    fn sample_rate_hz(&self) -> Option<&i32> { self.audio().sample_rate_hz() }
    fn bit_depth(&self) -> Option<&i16> { self.audio().bit_depth() }
    fn bitrate_kbps(&self) -> Option<&i32> { self.audio(). bitrate_kbps() }
    fn channel_layout(&self) -> Option<&str> { self.audio().channel_layout() }
    fn is_lossless(&self) -> &bool { self.audio().is_lossless() }
    fn loudness_lufs(&self) -> Option<&Decimal> { self.audio().loudness_lufs() }
    fn dynamic_range_db(&self) -> Option<&Decimal> { self.audio().dynamic_range_db() }

    // Content
    fn is_dialogue_driven(&self) -> &bool { self.audio().is_dialogue_driven() }
    fn has_transcript(&self) -> &bool { self.audio().has_transcript() }
    fn spoken_language_id(&self) -> Option<&i32> { self.audio().spoken_language_id() }
    fn recorded_on(&self) -> Option<&NaiveDate> { self.audio().recorded_on() }
    fn recording_venue(&self) -> Option<&str> { self.audio().recording_venue() }
}

impl HasAudio for Audio {
    fn audio(&self) -> &Audio { self }

    // File Metadata
    fn duration_seconds(&self) -> Option<&i32> { self.duration_seconds.as_ref() }
    fn recording_type(&self) -> &RecordingType { &self.recording_type }
    fn sample_rate_hz(&self) -> Option<&i32> { self.sample_rate_hz.as_ref() }
    fn bit_depth(&self) -> Option<&i16> { self.bit_depth.as_ref() }
    fn bitrate_kbps(&self) -> Option<&i32> { self. bitrate_kbps.as_ref() }
    fn channel_layout(&self) -> Option<&str> { self.channel_layout.as_deref() }
    fn is_lossless(&self) -> &bool { &self.is_lossless }
    fn loudness_lufs(&self) -> Option<&Decimal> { self.loudness_lufs.as_ref() }
    fn dynamic_range_db(&self) -> Option<&Decimal> { self.dynamic_range_db.as_ref() }

    // Content
    fn is_dialogue_driven(&self) -> &bool { &self.is_dialogue_driven }
    fn has_transcript(&self) -> &bool { &self.has_transcript }
    fn spoken_language_id(&self) -> Option<&i32> { self.spoken_language_id.as_ref() }
    fn recorded_on(&self) -> Option<&NaiveDate> { self.recorded_on.as_ref() }
    fn recording_venue(&self) -> Option<&str> { self.recording_venue.as_deref() }
}

impl NonRootStruct for Audio {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        todo!()
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            media: Default::default(),
            duration_seconds: Default::default(), recording_type: Default::default(), sample_rate_hz: Default::default(),
            bit_depth: Default::default(), bitrate_kbps: Default::default(), channel_layout: Default::default(),
            is_lossless: Default::default(), loudness_lufs: Default::default(), dynamic_range_db: Default::default(),
            is_dialogue_driven: Default::default(), has_transcript: Default::default(), spoken_language_id: Default::default(),
            recorded_on: Default::default(), recording_venue: Default::default() }
    }
}