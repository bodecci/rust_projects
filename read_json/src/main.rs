use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    //setup a raw string literal with r#", in this case a json formatted
    let json = r#"
    {
        "article": "how to work with json in Rust",
        "author": "Bode",
        "paragraph": [
          {
            "name": "starting sentences"
            },
            {
            "name": "body of the paragraph"
            },
            {
            "name": "end of the paragraph"
            }
          ]
        }
        "#;
    let parsed : Article = read_json_tyed(json);

    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}

fn read_json_tyed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}