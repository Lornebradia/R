use crate::{lang::Signal, parser::*};

pub type HighlightResult = Result<Vec<(String, Style)>, Signal>;
pub trait LocalizedParser: std::marker::Sync {
    fn parse_input(&self, input: &str) -> ParseResult;
    fn parse_highlight(&self, input: &str) -> HighlightResult;
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Copy, Clone, Default, PartialEq, clap::ValueEnum, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Localization {
    #[default]
    En,
    Es,
    Cn,
    #[value(skip)]
    Pirate,
    #[value(skip)]
    Emoji,
}

impl LocalizedParser for Localization {
    fn parse_input(&self, input: &str) -> ParseResult {
        use Localization::*;
        match self {
            En => LocalizedParser::parse_input(&en::Parser, input),
            Es => LocalizedParser::parse_input(&es::Parser, input),
            Cn => LocalizedParser::parse_input(&cn::Parser, input),
            Pirate => LocalizedParser::parse_input(&pirate::Parser, input),
            Emoji => LocalizedParser::parse_input(&emoji::Parser, input),
        }
    }

    fn parse_highlight(&self, input: &str) -> HighlightResult {
        use Localization::*;
        match self {
            En => LocalizedParser::parse_highlight(&en::Parser, input),
            Es => LocalizedParser::parse_highlight(&es::Parser, input),
            Cn => LocalizedParser::parse_highlight(&cn::Parser, input),
            Pirate => LocalizedParser::parse_highlight(&pirate::Parser, input),
            Emoji => LocalizedParser::parse_highlight(&emoji::Parser, input),
        }
    }
}
