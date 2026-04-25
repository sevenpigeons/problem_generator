
use std::str::FromStr;

use axum::{http::{HeaderMap, header}, response::IntoResponse};
use genpdf::{Document, elements::{self, Paragraph}, style};
use crate::GenpdfState;
use strum_macros::{Display, EnumString};


#[derive(Debug,EnumString,Display,PartialEq)]
pub enum Subject {
    Math,
    Physics
}

#[derive(Debug,Display,PartialEq)]
pub enum Theme {
    Math(MathTheme),
    Physics(PhysTheme)
}


#[derive(Debug,EnumString,Display,PartialEq)]
pub enum PhysTheme {
    LawMotion,
    Momentum,
    Energy,
}

#[derive(Debug,EnumString,Display,PartialEq)]
pub enum MathTheme {
    ParametricEquations,
    InfiniteSequenceSeries,
    PartialDerivatives,
    
}

#[derive(Debug,PartialEq)]
pub struct Question {
    pub subject: Subject,
    pub theme: Theme,
    pub text: String
}

#[derive(Debug,Display)]
pub enum QuestionParseError {
    QuestionFileParseError,
    Strum(strum::ParseError)
}

impl FromStr for Question {
    type Err = QuestionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l : Vec<&str> = s.splitn(3, '\n').collect();
        let subj: Subject = Subject::from_str(l[0]).unwrap();
        let theme: Theme = match subj {
            Subject::Math => Theme::Math(MathTheme::from_str(l[1]).unwrap()),
            Subject::Physics => Theme::Physics(PhysTheme::from_str(l[1]).unwrap()),
        };
        if !l[2].is_empty() {
        Ok(Question { subject: subj, theme: theme, text: l[2].to_string() })
        } else {
            Err(QuestionParseError::QuestionFileParseError)
        }
    }
    
}


pub fn build_document_base(state:GenpdfState) -> Document {

    let mut doc = genpdf::Document::new(state.font_family);
    doc.set_title("Test document title");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc.set_hyphenator(state.hyphenator);
    let mut title = Paragraph::default();
    title.push_styled("title", style::Effect::Bold);
    title.set_alignment(genpdf::Alignment::Center);
    doc.push(title);

    doc.push(elements::Break::new(5));
    doc
}

pub fn build_response(doc: Document) -> impl IntoResponse {

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    let mut w: Vec<u8> = vec![];
    let _ = doc.render(&mut w).expect("error during pdf rendering");
    (headers,w)
}
