use xmltree::{Element};
use std::fs::File;
use std::io::{Read};
fn main(){
    // read and remove special characters
    let mut file = File::open("dict.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let xml_string = xml_string.replace("&#4;", "");

    // parse the xml file
    let root = Element::parse(xml_string.as_bytes()).unwrap();
    let children = root.children;
    let batch_size = 1;
    for (_i, chunk)  in children.chunks(batch_size).enumerate()  {
        let mut root = Element::new("LISTOFLEDGERS");
        root.children = chunk.to_vec();
        println!("{:?}",root);  

    }
}

    // println!("{:#?}", vec[1]);


