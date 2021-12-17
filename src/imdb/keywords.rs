#![warn(clippy::all)]

use super::error::Err;
use crate::Res;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use std::borrow::Cow;
use std::fmt;

#[derive(Clone)]
pub struct KeywordSet {
  keywords: Vec<String>,
  matcher: AhoCorasick,
}

impl fmt::Display for KeywordSet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for keyword in &self.keywords {
      write!(f, "`{}` ", keyword)?
    }

    Ok(())
  }
}

impl KeywordSet {
  fn add_keyword(keywords: &mut Vec<String>, keyword: Cow<str>) {
    if keyword.is_empty() || keyword.len() == 1 {
      return;
    }

    let keyword = keyword.into_owned();
    if !keywords.contains(&keyword) {
      keywords.push(keyword);
    }
  }

  fn add_special_keyword(keywords: &mut Vec<String>, keyword: &str) {
    if keyword == "$h!t" || keyword == "sh!t" || keyword == "$hit" {
      Self::add_keyword(keywords, Cow::from("shit"));
    }
  }

  pub fn new_query_keywordset(s: &str) -> Res<Self> {
    let mut keywords = vec![];

    if s.is_empty() {
      return Err::bad_keywords();
    }

    for keyword in s.split_whitespace() {
      Self::add_keyword(&mut keywords, Cow::from(keyword));
    }

    if keywords.is_empty() {
      return Err::bad_keywords();
    }

    let matcher = AhoCorasickBuilder::new().auto_configure(&keywords).build(&keywords);

    Ok(Self { keywords, matcher })
  }

  pub fn make_title_keywordset(s: &str) -> Vec<String> {
    let mut keywords = vec![];

    for keyword in s.split_whitespace() {
      for keyword in keyword.split(&['\'', '-'][..]) {
        Self::add_special_keyword(&mut keywords, keyword);
        Self::add_keyword(&mut keywords, Cow::from(keyword));
      }

      Self::add_special_keyword(&mut keywords, keyword);
      Self::add_keyword(&mut keywords, Cow::from(keyword));
    }

    keywords
  }

  pub fn matches(&self, text: &str) -> bool {
    let mut matches: Vec<usize> = self.matcher.find_iter(text).map(|mat| mat.pattern()).collect();
    matches.sort_unstable();
    matches.dedup();
    matches.len() == self.keywords.len()
  }
}
