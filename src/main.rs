use std::fs::{File};
use std::fs;
use std::path::Path;
use std::io;
use std::process::exit;
use xml::reader::{XmlEvent, EventReader};

#[derive(Debug)]
pub enum SEError {
    Io(std::io::Error),
    Xml(xml::reader::Error),
}

impl From<io::Error> for SEError {
    fn from (err: io::Error) -> Self {
        SEError::Io(err)
    }
}

impl From<xml::reader::Error> for SEError {
    fn from (err: xml::reader::Error) -> Self {
        SEError::Xml(err)
    }
}

fn read_entire_xml_file<P>(file_path: P) -> Result<String, SEError> 
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        match event? {
            XmlEvent::Characters(text) => {
                content.push_str(&text);
            }
            _=> {}
        }
    }

    Ok(content)
}

fn main() -> io::Result<()> {
    let target_path = "docs.gl/gl4";
    let dir = fs::read_dir(target_path)?;
    for file in dir {
        let file_path = file?.path();
        let content = read_entire_xml_file(&file_path).unwrap_or_else(|err| {
            eprintln!("ERROR: could not read file {file_path:?} with {err:?}");
            exit(1)
        });

        println!("{file_path:?} -> {size}", size = content.len());
    }


    Ok(())
}
