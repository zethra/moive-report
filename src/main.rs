use anyhow::Result;
use cstr::*;
use horrorshow::html;
use qmetaobject::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
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

type Movies = Vec<(String, Vec<Movie>)>;

fn load_csv(path: impl AsRef<Path>) -> Result<Movies> {
    let csv_file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(csv_file);
    let mut movies = HashMap::new();
    for result in rdr.deserialize() {
        let movie: Movie = result?;
        movies
            .entry(movie.category.clone())
            .and_modify(|m: &mut Vec<_>| m.push(movie))
            .or_insert(Vec::new());
    }
    let mut movie_list: Vec<_> = movies.into_iter().collect();
    movie_list.sort_by(|l, r| l.0.cmp(&r.0));
    for (_, movies) in &mut movie_list {
        movies.sort_by(|l, r| l.title.cmp(&r.title));
    }
    Ok(movie_list)
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
    open: qt_method!(fn(&mut self, path: QString) -> bool),
    save: qt_method!(fn(&self, path: QString)),
    has_error: qt_property!(bool; NOTIFY has_error_changed),
    error_msg: qt_property!(QString; NOTIFY error_msg_changed),

    has_error_changed: qt_signal!(),
    error_msg_changed: qt_signal!(),
}

impl UI {
    fn open(&mut self, path: impl Into<QString>) -> bool {
        let path: String = path.into().into();
        match load_csv(&path[7..]) {
            Ok(movies) => {
                self.report_html = generate_report(&movies);
                true
            }
            Err(e) => {
                self.error_msg = e.to_string().into();
                self.has_error = true;
                self.error_msg_changed();
                self.has_error_changed();
                false
            }
        }
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
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 || args.len() == 3 {
        let in_path = Path::new(&args[1]);
        let out_path = if args.len() == 3 {
            Path::new(&args[2])
        } else {
            Path::new("./Video Collection.html")
        };
        match load_csv(in_path) {
            Ok(movies) => {
                let report_html = generate_report(&movies);
                fs::write(&out_path, &report_html).expect("Failed to write file");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    } else {
        init_qml_resources();
        qml_register_type::<UI>(cstr!("UI"), 1, 0, cstr!("UI"));
        let mut engine = QmlEngine::new();
        engine.load_file("qrc:/src/ui.qml".into());
        engine.exec();
    }
}
