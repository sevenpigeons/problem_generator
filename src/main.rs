use genpdf::{elements::{self, Paragraph}, fonts::FontData, style};
use hyphenation::{Load, Standard};


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

    // this block should be really moved somewhere in a function
    let mut doc = genpdf::Document::new(state.font_family);
    doc.set_title("Test document title");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    let mut title = Paragraph::default();
    title.push_styled("title", style::Effect::Bold);
    title.set_alignment(genpdf::Alignment::Center);
    doc.push(title);



    doc.push(elements::Break::new(5));
    doc.set_hyphenator(state.hyphenator);

    for i in 0..len  {
        doc.push(
            genpdf::elements::Paragraph::new(
                format!("this is line {}", i)
            ));


    }

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());



    let mut w: Vec<u8> = vec![];
    let _ = doc.render(&mut w).expect("error during pdf rendering");
    (headers,w.clone())
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
