use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use sqlx::query_scalar;

use crate::models::{enums::print::BookType::{self, PictureBook}, property_structs::print::{HasPrint, Print}, root_structs::media::HasMedia, traits::NonRootStruct};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Book {
    print:                 Print,

    book_type:             BookType,
    volume_number:         Option<Decimal>,
    series_position:       Option<Decimal>,
    original_published_on: Option<NaiveDate>,
    is_abridged:           bool,
    is_annotated:          bool,
    dewey_decimal:         Option<String>,
    library_of_congress:   Option<String>,
    subject_headings:      Vec<String>,
}

impl HasMedia for Book {
    fn media(&self) -> &crate::models::root_structs::media::Media {
        self.print.media()
    }
}

impl HasPrint for Book {
    fn print(&self) -> &Print {
        &self.print
    }
}

/// Provides Getters for innermost Book fields while also defining the canonical path to them
pub trait HasBook 
where
    Self: HasPrint
{
    fn book(&self) -> &Book;

    fn book_type(&self) -> &BookType { self.book().book_type()}
    fn volume_number(&self) -> Option<&Decimal> { self.book().volume_number()}
    fn series_position(&self) -> Option<&Decimal> { self.book().series_position()}
    fn original_published_on(&self) -> Option<&NaiveDate> { self.book().original_published_on() }
    fn is_abridged(&self) -> &bool { self.book().is_abridged() }
    fn is_annotated(&self) -> &bool { self.book().is_annotated() }
    fn dewey_decimal(&self) -> Option<&str> { self.book().dewey_decimal() }
    fn library_of_congress(&self) -> Option<&str> { self.book().library_of_congress() }
    fn subject_headings(&self) -> &[String] { self.book().subject_headings() }

}

impl HasBook for Book {
    fn book(&self) -> &Book { self }

    fn book_type(&self) -> &BookType { &self.book_type }
    fn volume_number(&self) -> Option<&Decimal> { self.volume_number.as_ref() }
    fn series_position(&self) -> Option<&Decimal> { self.series_position.as_ref() }
    fn original_published_on(&self) -> Option<&NaiveDate> { self.original_published_on.as_ref() }
    fn is_abridged(&self) -> &bool { &self.is_abridged }
    fn is_annotated(&self) -> &bool { &self.is_annotated }
    fn dewey_decimal(&self) -> Option<&str> { self.dewey_decimal.as_deref() }
    fn library_of_congress(&self) -> Option<&str> { self.library_of_congress.as_deref() }
    fn subject_headings(&self) -> &[String] { &self.subject_headings }
}

impl NonRootStruct for Book {
    async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        query_scalar!(
        r#"
            INSERT INTO book (
            id,
            book_type, volume_number, series_position,
            original_published_on,
            is_abridged, is_annotated,
            dewey_decimal, library_of_congress,
            subject_headings
            )
            VALUES (
            $1,
            $2::book_type, $3, $4,
            $5,
            $6, $7,
            $8, $9, $10
            )
            "#,
            self.id(),
            self.book_type(), self.volume_number(), self.series_position(),
            self.original_published_on(),
            self.is_abridged(), self.is_annotated(),
            self.dewey_decimal(), self.library_of_congress(), self.subject_headings()
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(())
    }
}

impl Default for Book {

    /// Bokuyaba volume 1
    fn default() -> Self {
        Self { 
            print: Default::default(),
            book_type: PictureBook, 
            volume_number: Default::default(), series_position: Default::default(),
            original_published_on: Default::default(),
            is_abridged: Default::default(), is_annotated: Default::default(),
            dewey_decimal: Default::default(), library_of_congress: Default::default(),
            subject_headings: Default::default() }
    }
}