
use std::str::FromStr;

use genpdf::fonts::FontData;
use hyphenation::{Load, Standard};

mod util;
use crate::util::{Question, build_document_base, build_response};

use axum::{Router, extract::{Path, State}, http::{HeaderMap, header}, response::IntoResponse, routing::get};

#[derive(Clone)]
pub struct GenpdfState {
    pub font_family:genpdf::fonts::FontFamily<FontData>,
    pub hyphenator: Standard
}


async fn send_pdf(
    State(state):State<GenpdfState>,
    key: Option<Path<i32>>)
-> impl IntoResponse {


    let len = match key {
        Some(Path(a)) => a,
        None => 1,
    };

    let mut doc = build_document_base(state);

    let test_string: &str = "Math;
PartialDerivatives;

f(x,y) = x^ {{0}} + x^ {{1}} *y^ {{2}} - {{3}} *y^ {{4}}

f_x( {{5}} , {{6}} ) = ?

f_y( {{7}} , {{8}} ) = ?;
1,5
2,5
2,5
1,5
1,5
1,33
2,44
1,33
2,44";

    let q = Question::from_str(test_string).unwrap();

    for i in 0..len  {
        doc.push(
            genpdf::elements::Paragraph::new(
              //  format!("this is line {}", i)
                q.generate_question()
            ));
    }


    build_response(doc)
}


#[tokio::main]
async fn main() {

    let en_us = Standard::from_embedded(hyphenation::Language::EnglishUS).unwrap();
    let font_family = match genpdf::fonts::from_files("./fonts", "Roboto", None) {
        Ok(a) => a,
        Err(err) => panic!("errpr {err}")
    };

    let genpdf_state = GenpdfState {
        font_family:font_family,
        hyphenator:en_us
    };

    let routes = Router::new()
        .route("/{key}", get(send_pdf))
        .route("/", get(send_pdf))
        .with_state(genpdf_state);



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,routes).await.unwrap();


}




#[test]
fn test_string_to_question_parse() {
    let test_question = Question {
        subject:util::Subject::Math,
        theme:util::Theme::Math(util::MathTheme::ParametricEquations),
        text: "\n\n{{0}} + y = 12".to_string(),
        var_conditions:vec![(1,4)],
        ans_expression: None};
    let test_string: &str = "Math;
ParametricEquations;


{{0}} + y = 12;
1,4";

 let text = Question::from_str(test_string).unwrap().generate_question();
    println!("{}\n",text);
    assert_eq!(Question::from_str(test_string).unwrap(),test_question);

}
