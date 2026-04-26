
use axum::{http::{HeaderMap, header}, response::IntoResponse};
use dyn_fmt::AsStrFormatExt;
use genpdf::{Document, elements::{self, Paragraph}, style};
use rand::random_range;
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

pub struct Question<F>
    where F: Fn(&[f32])-> f32
    {
    pub subject: Subject,
    pub theme: Theme,
    pub text: &'static str,
    pub var_conditions: &'static [(i32,i32)],
    pub ans_expression: F
}

#[derive(Debug,Display)]
pub enum QuestionParseError {
    QuestionFileParseError,
    Strum(strum::ParseError)
}


impl<F> Question<F> 
    where F: Fn(&[f32])-> f32
{
    pub fn generate_question(&self) -> (String,String) {
        let vars:Vec<i32> = self.var_conditions.iter().map(|x| {
            random_range(x.0..x.1)
        }).collect();
        let text = self.text.format(&vars[..]);

        let answer = (self.ans_expression)(vars.iter().map(|x| *x as f32).collect::<Vec<f32>>().iter().as_slice());

        (text.to_string(),format!("{}",answer))
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
