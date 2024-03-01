use lazy_static::lazy_static;
use tera::Tera;

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

pub fn render_template(template_name: &str) -> String {
    let context = tera::Context::new();

    match TEMPLATES.render(template_name, &context) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Render error: {}", e);
            TEMPLATES.render("error.html", &context).unwrap()
        }
    }
}
