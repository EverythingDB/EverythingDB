use crate::models::basic_types::book::HasBook;
use crate::models::property_structs::audio::HasAudio;
use crate::models::property_structs::print::HasPrint;
use crate::models::traits::NonRootStruct;
use crate::models::{basic_types::book::Book, enums::audiovisual::NarrationStyle, property_structs::audio::Audio};
use crate::models::root_structs::media::{HasMedia, Media};

pub struct AudioBook{
    audio:             Audio,
    source_book:       Book,

    narration_style:   NarrationStyle,
    is_dramatized:     bool,
    has_sound_design:  bool,
}

impl HasMedia for AudioBook {
    fn media(&self) -> &Media {
        self.audio.media()
    }
}

impl HasAudio for AudioBook {
    fn audio(&self) -> &Audio {
        &self.audio
    }
}

impl HasPrint for AudioBook {
    fn print(&self) -> &crate::models::property_structs::print::Print {
        self.source_book.print()
    }
}

impl HasBook for AudioBook {
    fn book(&self) -> &Book {
        &self.source_book
    }
}

pub trait HasAudioBook: HasBook+HasAudio {
    fn audiobook(&self) -> &AudioBook;

    fn narration_style(&self) -> &NarrationStyle { self.audiobook().narration_style() }
    fn is_dramatized(&self) -> &bool { self.audiobook().is_dramatized() }
    fn has_sound_design(&self) -> &bool { self.audiobook().has_sound_design()}
}

impl HasAudioBook for AudioBook {
    fn audiobook(&self) -> &AudioBook { self }

    fn narration_style(&self) -> &NarrationStyle { &self.narration_style }
    fn is_dramatized(&self) -> &bool { &self.is_dramatized }
    fn has_sound_design(&self) -> &bool { &self.has_sound_design }
}

impl NonRootStruct for AudioBook {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        todo!()
    }
}

impl Default for AudioBook{
    fn default() -> Self {
        Self {
            audio: Audio::default(), source_book: Book::default(),
            narration_style: NarrationStyle::default(),
            is_dramatized: Default::default(), has_sound_design: Default::default() }
    }
}