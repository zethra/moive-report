use cstr::*;
use horrorshow::html;
use qmetaobject::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Movie {
    title: String,
    category: String,
    rating: String,
    actors: String,
    aspect: String,
    format: String,
}

type Movies = HashMap<String, Vec<Movie>>;

fn load_csv(path: impl AsRef<Path>) -> Movies {
    let csv_file = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(csv_file);
    let mut movies = HashMap::new();
    for result in rdr.deserialize() {
        let movie: Movie = result.unwrap();
        if !movies.contains_key(&movie.category) {
            movies.insert(movie.category.clone(), Vec::new());
        }
        movies.get_mut(&movie.category).unwrap().push(movie);
    }
    movies
}

fn generate_report(movies: &Movies) -> String {
    (html! {
        html {
            head {
                style : include_str!("style.css");
                meta (charset="UTF-8");
            }
            body {
                h1 (align="center") : "The Goldberg's Video Collection";

                @ for (category, movie_list) in movies {
                    h2 : format!("Category: {}", category);
                    table {
                        tr {
                            th : "Title";
                            th : "Rating";
                            th : "Actors";
                            th : "Aspect";
                            th : "Format";
                        }
                        @ for movie in movie_list {
                            tr {
                                td : &movie.title;
                                td : &movie.rating;
                                td : &movie.actors;
                                td : &movie.aspect;
                                td : &movie.format;
                            }
                        }
                    }
                }
            }
        }
    })
    .to_string()
}

#[derive(QObject, Default)]
struct UI {
    base: qt_base_class!(trait QObject),
    report_html: String,
    open: qt_method!(fn(&mut self, path: QString)),
    save: qt_method!(fn(&self, path: QString)),
}

impl UI {
    fn open(&mut self, path: impl Into<QString>) {
        let path: String = path.into().into();
        println!("CSV: {}", &path[7..]);
        let movies = load_csv(&path[7..]);
        self.report_html = generate_report(&movies);
    }

    fn save(&self, path: impl Into<QString>) {
        let path: String = path.into().into();
        fs::write(&path[7..], &self.report_html).unwrap();
    }
}

qrc!(init_qml_resources,
    "src" {
        "src/ui.qml" as "ui.qml",
    }
);

fn main() {
    init_qml_resources();
    qml_register_type::<UI>(cstr!("UI"), 1, 0, cstr!("UI"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/src/ui.qml".into());
    engine.exec();
}
