use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
use xml::reader::{EventReader, XmlEvent};
use std::io::Cursor;

use quickxml_to_serde::{xml_string_to_json, Config, NullValue};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XMI {
    DAnalysis: DAnalysis,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DAnalysis {
    // semanticResources: Vec<String>,
    semanticResources: ()
}

fn main() {
    let src = std::fs::read_to_string("In-Flight Entertainment System.aird").unwrap();
    let c = Cursor::new(src.clone());

    let parser = EventReader::new(c);
    let mut depth = 0;
    for e in parser.into_iter().take(100) { 
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    let conf = Config::new_with_defaults();
    let json = xml_string_to_json(src.to_owned(), &conf).unwrap();
    let json_str = json.to_string();
    std::fs::write("out.json", json_str).unwrap();

    let xmi: XMI = serde_json::value::from_value(json).unwrap();
    dbg!(xmi);
}
