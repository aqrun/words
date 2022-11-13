use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pronunciation {
    pub b: Option<String>,
    pub a: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Concise {
    pub n: Option<String>,
    pub adj: Option<String>,
    pub v: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordItem {
    pub p: Option<Pronunciation>,
    pub c: Option<Concise>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordItemDetail {
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordData {
    pub name: String,
    pub pronunciations: Vec<WordItemDetail>,
    pub concises: Vec<WordItemDetail>,
}

impl WordData {
    pub fn from_word_item(
        word_item_data: &HashMap<String, WordItem>
    ) -> Vec<WordData> {
        let mut items: Vec<WordData> = Vec::new();

        for (k, v) in word_item_data.iter() {
            let mut p: Vec<WordItemDetail> = Vec::new();
            let mut c: Vec<WordItemDetail> = Vec::new();

            if let Some(p_data) = &v.p {
                if let Some(desc) = &p_data.b {
                    p.push(WordItemDetail {
                        name: String::from("b"),
                        desc: desc.into(),
                    });
                }
                if let Some(desc) = &p_data.a {
                    p.push(WordItemDetail {
                        name: String::from("a"),
                        desc: desc.into(),
                    });
                }
            }

            if let Some(c_data) = &v.c {
                if let Some(desc) = &c_data.n {
                    c.push(WordItemDetail {
                        name: String::from("n"),
                        desc: desc.into(),
                    });
                }
                if let Some(desc) = &c_data.v {
                    c.push(WordItemDetail {
                        name: String::from("v"),
                        desc: desc.into(),
                    });
                }
                if let Some(desc) = &c_data.adj {
                    c.push(WordItemDetail {
                        name: String::from("adj"),
                        desc: desc.into(),
                    });
                }
            }

            let item = Self {
                name: String::from(k),
                pronunciations: p,
                concises: c,
            };
            items.push(item);
        }
        
        items
    }
}