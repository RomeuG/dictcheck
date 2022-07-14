use owo_colors::colors::xterm::UserBrightCyan;
use owo_colors::colors::xterm::UserBrightMagenta;
use owo_colors::colors::xterm::UserBrightRed;
use owo_colors::colors::xterm::UserBrightYellow;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Response {
    Ok(Root),
    Err(ResponseError),
}

// type Root = Vec<Root2>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root(Vec<Root2>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root2 {
    word: String,
    phonetic: Option<String>,
    phonetics: Vec<Phonetic>,
    origin: Option<String>,
    meanings: Vec<Meaning>,
    license: License2,
    #[serde(rename = "sourceUrls")]
    source_urls: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Phonetic {
    audio: String,
    #[serde(rename = "sourceUrl")]
    source_url: Option<String>,
    license: Option<License>,
    text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct License {
    name: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Definition {
    definition: String,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
    example: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct License2 {
    name: String,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResponseError {
    title: String,
    message: String,
    resolution: String,
}

#[derive(Debug)]
enum Error {
    RequestError(reqwest::Error),
    JsonParseError(reqwest::Error),
}

const URL: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";

fn print(object: &Root) {
    let root = &object.0;

    for entry in root {
        println!("* {}", "word".bold().fg::<UserBrightRed>());
        if entry.phonetic.is_some() {
            println!("\t{} ({})\n", entry.word, entry.phonetic.as_ref().unwrap());
        } else {
            println!("\t{}\n", entry.word);
        }

        if entry.origin.is_some() {
            println!("Origin: {}", entry.origin.as_ref().unwrap())
        }

        if !entry.phonetics.is_empty() {
            println!("* {}", "phonetics".bold().fg::<UserBrightCyan>());

            for phonetic in &entry.phonetics {
                if phonetic.text.is_some() {
                    println!("\t{}: {}", "text".bold(), phonetic.text.as_ref().unwrap());
                }

                if phonetic.audio != "" {
                    println!("\t{}: {}", "audio".bold(), phonetic.audio);
                }

                if phonetic.source_url.is_some() {
                    println!(
                        "\t{}: {}",
                        "source".bold(),
                        phonetic.source_url.as_ref().unwrap()
                    );
                }

                println!();
            }
        }

        for meaning in &entry.meanings {
            println!(
                "* {}",
                meaning.part_of_speech.bold().fg::<UserBrightYellow>()
            );

            if !meaning.synonyms.is_empty() {
                println!("\t{}: {:?}", "synonyms".bold(), meaning.synonyms);
            }

            if !meaning.antonyms.is_empty() {
                println!("\t{}: {:?}", "antonyms".bold(), meaning.antonyms);
            }

            for definition in &meaning.definitions {
                println!("\t{}: {}", "definition".bold(), definition.definition);

                if definition.example.is_some() {
                    println!(
                        "\t{}: {}",
                        "example".bold(),
                        definition.example.as_ref().unwrap()
                    );
                }

                if !definition.synonyms.is_empty() {
                    println!("\t{}: {:?}", "synonyms".bold(), definition.synonyms);
                }

                if !definition.antonyms.is_empty() {
                    println!("\t{}: {:?}", "antonyms".bold(), definition.antonyms);
                }

                println!();
            }
        }

        if !entry.source_urls.is_empty() {
            println!(
                "* {}\n\t{:?}\n",
                "urls".bold().fg::<UserBrightMagenta>(),
                entry.source_urls
            );
        }
    }
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Incorrect number of arguments!");
        std::process::exit(1);
    }

    let word = args.get(1).unwrap();
    let url = format!("{}/{}", URL, word);

    let response = reqwest::blocking::get(&url)
        .map_err(Error::RequestError)?
        .json::<Response>()
        .map_err(Error::JsonParseError)?;

    match response {
        Response::Ok(success) => {
            print(&success);
        }
        Response::Err(error) => {
            println!("Error: {}", error.message);
        }
    };

    Ok(())
}
