use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragrap {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}