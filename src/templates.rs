use axum::{routing::get, Router};
use lazy_static::lazy_static;
use std::sync::Arc;
use tera::Tera;

use crate::AppState;

mod index;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("./templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera
    };
}

pub fn render_template(template_name: &str, context: tera::Context) -> String {
    match TEMPLATES.render(template_name, &context) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Render error: {}", e);
            TEMPLATES.render("error.html", &context).unwrap()
        }
    }
}

pub fn render_templates(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index::index))
        .with_state(state)
}
