use serde::{Serialize, Deserialize};

use crate::models::{enums::print::{ProseFormat, ReadingDirection}, root_structs::media::{HasMedia, Media}, traits::NonRootStruct};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Print {
    media:                     Media,

    // counts
    page_count:                Option<i32>,
    word_count:                Option<i32>,
    character_count:           Option<i32>,
    illustration_count:        Option<i32>,

    // text formating and languge
    prose_format:              ProseFormat,
    reading_direction:         ReadingDirection,
    script_language_id:        Option<i32>,

    // flags
    is_illustrated:            bool,
    is_translation:            bool,
    has_furigana:              bool,
    has_footnotes:             bool,
    has_index:                 bool,
    has_bibliography:          bool,

    // general info
    reading_level:             Option<String>, // Lexile, JLPT, CEFR, grade
    estimated_reading_minutes: Option<i32>,
    translated_from_id:        Option<i32>
}

impl HasMedia for Print {
    fn media(&self) -> &Media {
        &self.media
    }
}

/// Provides Getters for innermost Print fields while also defining the canonical path to them
pub trait HasPrint: HasMedia{
    fn print(&self) -> &Print;

    // counts
    fn page_count(&self) -> Option<&i32> { self.print().page_count()}
    fn word_count(&self) -> Option<&i32> { self.print().word_count() }
    fn character_count(&self) -> Option<&i32> { self.print().character_count() }
    fn illustration_count(&self) -> Option<&i32> { self.print().illustration_count() }

    // text formating and language
    fn prose_format(&self) -> &ProseFormat { self.print().prose_format() }
    fn reading_direction(&self) -> &ReadingDirection { self.print().reading_direction() }
    fn script_language_id(&self) -> Option<&i32> { self.print().script_language_id() }

    // flags
    fn is_illustrated(&self) -> &bool { self.print().is_illustrated() }
    fn is_translation(&self) -> &bool { self.print().is_translation() }
    fn has_furigana(&self) -> &bool { self.print().has_furigana() }
    fn has_footnotes(&self) -> &bool { self.print().has_footnotes() }
    fn has_index(&self) -> &bool { self.print().has_index() }
    fn has_bibliography(&self) -> &bool { self.print().has_bibliography() }

    // general info
    fn reading_level(&self) -> Option<&str> { self.print().reading_level() }
    fn estimated_reading_minutes(&self) -> Option<&i32> { self.print().estimated_reading_minutes() }
    fn translated_from_id(&self) -> Option<&i32> { self.print().translated_from_id() }
}

impl HasPrint for Print {
    fn print(&self) -> &Print { self }

    // counts
    fn page_count(&self) -> Option<&i32> { self.page_count.as_ref()}
    fn word_count(&self) -> Option<&i32> { self.word_count.as_ref() }
    fn character_count(&self) -> Option<&i32> { self.character_count.as_ref() }
    fn illustration_count(&self) -> Option<&i32> { self.illustration_count.as_ref() }

    // text formating and language
    fn prose_format(&self) -> &ProseFormat { &self.prose_format }
    fn reading_direction(&self) -> &ReadingDirection { &self.reading_direction }
    fn script_language_id(&self) -> Option<&i32> { self.script_language_id.as_ref() }

    // flags
    fn is_illustrated(&self) -> &bool { &self.is_illustrated }
    fn is_translation(&self) -> &bool { &self.is_translation }
    fn has_furigana(&self) -> &bool { &self.has_furigana }
    fn has_footnotes(&self) -> &bool { &self.has_footnotes }
    fn has_index(&self) -> &bool { &self.has_index }
    fn has_bibliography(&self) -> &bool { &self.has_bibliography }

    // general info
    fn reading_level(&self) -> Option<&str> { self.reading_level.as_deref() }
    fn estimated_reading_minutes(&self) -> Option<&i32> { self.estimated_reading_minutes.as_ref() }
    fn translated_from_id(&self) -> Option<&i32> { self.translated_from_id.as_ref() }
}

impl NonRootStruct for Print {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        todo!()
    }
}

impl Default for Print {

    /// Bokuyaba Volume 1
    fn default() -> Self {
        Self {
            media: Default::default(), // change
            page_count: Default::default(), word_count: Default::default(), character_count: Default::default(), illustration_count: Default::default(),
            prose_format: Default::default(), reading_direction: Default::default(), script_language_id: Default::default(),
            is_illustrated: Default::default(), is_translation: Default::default(), has_furigana: Default::default(), has_footnotes: Default::default(), has_index: Default::default(), has_bibliography: Default::default(),
            reading_level: Default::default(), estimated_reading_minutes: Default::default(), translated_from_id: Default::default() }
    }
}
