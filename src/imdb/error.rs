#![warn(clippy::all)]

use crate::Res;
use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display)]
#[display(fmt = "{}")]
pub enum DbErr {
  #[display(fmt = "ID does not start with `tt` (e.g. ttXXXXXXX)")]
  Id,
  #[display(fmt = "ID does not contain a number (e.g. ttXXXXXXX)")]
  IdNumber,
  #[display(fmt = "Unknown title type")]
  TitleType,
  #[display(fmt = "Invalid adult marker")]
  Adult,
  #[display(fmt = "Start year is not a number")]
  StartYear,
  #[display(fmt = "End year is not a number")]
  EndYear,
  #[display(fmt = "Runtime minutes is not a number")]
  RuntimeMinutes,
  #[display(fmt = "Invalid genre")]
  Genre,
  #[display(fmt = "Unexpected end of file")]
  Eof,
  #[display(fmt = "Duplicate IMDB ID: {}", _0)]
  Duplicate(u64),
}

impl DbErr {
  pub(crate) fn id<T>() -> Res<T> {
    Err(Box::new(DbErr::Id))
  }

  pub(crate) fn adult<T>() -> Res<T> {
    Err(Box::new(DbErr::Adult))
  }

  pub(crate) fn eof<T>() -> Res<T> {
    Err(Box::new(DbErr::Eof))
  }

  pub(crate) fn duplicate<T>(id: u64) -> Res<T> {
    Err(Box::new(DbErr::Duplicate(id)))
  }
}

impl Error for DbErr {}
