use std::fs::File;
use std::process::exit;
use xml::reader::{XmlEvent, EventReader};

fn main() {
    let file_path = "docs.gl/gl2/glAccum.xhtml";
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {file_path}: {err}");
        exit(1);
    });

    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        let event = event.unwrap_or_else(|err| {
            eprintln!("ERROR: could not read next XML event: {err}");
            exit(1);
        });

        match event {
            XmlEvent::Characters(text) => {
                content.push_str(&text);
            }
            _=> {}
        }
    }

    println!("{content}");
}
