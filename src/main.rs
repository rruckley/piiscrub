
use rocket::form::Form;
use rocket::http::ContentType;
use rocket::State;
use rocket::fs::{FileServer,relative};

mod docs;
mod regex;

use crate::regex::RegexFilter;

#[macro_use] extern crate rocket;

#[derive(FromForm)]
struct InputData {
    text : String,
    action : String,
}

fn get_style() -> String {
    "
    <link rel=\"stylesheet\" href=\"/static/style.css\" type=\"text/css\" />
    ".to_owned()
}

async fn process_regex(regex : &State<RegexFilter>, context : String) -> Result<String,String> {
    Ok(regex.filter(context).expect("Could not get regex output"))
}

#[post("/process", data = "<form_data>")]
async fn process(regex : &State<RegexFilter>, form_data : Form<InputData>) -> (ContentType,String) {
    let action = &form_data.action;
    let style = get_style();
    let result : Result<String,String> = match action.as_str() {
        "regex" => {
            let reg = process_regex(regex, form_data.text.clone()).await;
            Ok(format!("<html><head><title>Regular Expressions</title>{}</head><body><h2>Regular Expressions</h2>{}</body></html>",style,reg.unwrap()))
        }
        _ => Ok(format!("Invalid Action: {}",action))
    };
    (ContentType::HTML,result.expect("Could not get output"))
}

#[get("/config")]
async fn config(regex : &State<RegexFilter>) -> (ContentType,String) {
    let result = regex.config().expect("Could not get doc config");
    (ContentType::HTML,result)
}

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML,"
    <html>
        <meta http-equiv=\"Refresh\" content=\"0; url='/static/index.html'\" />
    </html>")
}

#[launch]
async fn rocket() -> _ {
    let regex_filter = RegexFilter::new();
    rocket::build()
        .manage(regex_filter)
        .mount("/static",FileServer::from(relative!("static")))
        .mount("/",routes![index,process])
        .mount("/docs",routes![config])
}