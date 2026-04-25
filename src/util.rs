
use std::str::FromStr;

use axum::{http::{HeaderMap, header}, response::IntoResponse};
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

#[derive(Debug,PartialEq)]
pub struct Question {
    pub subject: Subject,
    pub theme: Theme,
    pub text: String,
    pub var_conditions: Vec<(i32,i32)>,
    pub ans_expression: Option<String>
}

#[derive(Debug,Display)]
pub enum QuestionParseError {
    QuestionFileParseError,
    Strum(strum::ParseError)
}

impl FromStr for Question {
    type Err = QuestionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l : Vec<&str> = s.splitn(5, ";\n").collect();
        let subj: Subject = Subject::from_str(l[0]).unwrap();
        let theme: Theme = match subj {
            Subject::Math => Theme::Math( match MathTheme::from_str(l[1]) {
                Ok(t) => t,
                Err(err) => panic!("{err}: string {0}",l[1])
            }),
            Subject::Physics => Theme::Physics(PhysTheme::from_str(l[1]).unwrap()),
        };
        if l[2].is_empty() {
            return Err(QuestionParseError::QuestionFileParseError);
        }
        let var_amount = l[2].to_string().matches("{{").collect::<Vec<_>>().len();
        if l[3].is_empty() {
            return Err(QuestionParseError::QuestionFileParseError);
        }
        let lines = l[3].lines().collect::<Vec<_>>();
        if !(lines.len() == var_amount) {
            return Err(QuestionParseError::QuestionFileParseError);
        }

        let var_conf:Vec<(i32,i32)> = lines.iter().map(|x| {
            let elements:Vec<_> = x.split(",").collect();
            (elements[0].parse().unwrap(),elements[1].parse().unwrap())
        }
        ).collect();

        if l.len() < 4 {
            Ok(Question { subject: subj, theme: theme, text: l[2].to_string(),var_conditions:var_conf ,ans_expression: Some(l[4].to_string()) })
        } else {
            Ok(Question { subject: subj, theme: theme, text: l[2].to_string(),var_conditions:var_conf ,ans_expression: None })
        }
    }

}


impl Question {
    pub fn generate_question(&self) -> String {
        let  text = self.text.split_whitespace().map(|x| {
            if x.contains("{{") {
                if x[0..2] == *"{{" && x[3..5] == *"}}" {
                    let var_num:usize = x.chars().nth(2).unwrap().to_string().parse().unwrap();
                    return format!("{}",random_range(self.var_conditions[var_num].0..self.var_conditions[var_num].1)).to_string();
                }
            }
            x.to_string()
        }).collect();

        text
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
