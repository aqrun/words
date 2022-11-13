extern crate tera;
#[macro_use]
extern crate lazy_static;

use word_item::{WordItem, WordData};
use std::collections::HashMap;
use tera::{Tera, Context};

mod word_item;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}


fn main() {
    println!("templates: {:?}", TEMPLATES.get_template_names().collect::<Vec<_>>());
    let yaml_data = include_str!("..\\words1113.yml");

    let words: HashMap<String, WordItem> = serde_yaml::from_str(yaml_data)
        .unwrap();
    // println!("{:?}", words);
    let word_data_list = WordData::from_word_item(&words);
    // println!("{:?}", word_data_list);
    let mut ctx = Context::new();
    ctx.insert("words", &word_data_list);

    let html = TEMPLATES.render("words.html", &ctx)
        .unwrap();

    std::fs::write("target\\words.html", &html).unwrap();
    println!("Generate Complete!!");
}