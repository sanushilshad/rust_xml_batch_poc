use xmltree::{Element, XMLNode};
use std::fs::File;
use std::io::Read;
fn main(){
    // read and remove special characters
    let mut file = File::open("dict.xml").unwrap();
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string).unwrap();
    let xml_string = xml_string.replace("&#4;", "");

    // parse the xml file
    let mut root = Element::parse(xml_string.as_bytes()).unwrap();
    let children = root.children.clone();
    

    // calculating slice index based on the batch size and children node length
    let batch_size = 1;
    let children_len= children.len() as i32;
    let mut start_index = 0;
    let mut end_index = batch_size as usize;
    let limit= (children_len as f64/ batch_size as f64).ceil();

    let mut vec: Vec<XMLNode> = Vec::new();

    // create xm by batch
    for i in 1..=limit as i32 {
        println!("Start: {}, End: {}", start_index, end_index);
        root.children.clear();
        let sliced = &children[start_index..end_index];
        root.children = sliced.to_vec();
        vec.push(XMLNode::Element(root.clone()));

        start_index = end_index;
        end_index = ((i) + batch_size) as usize;


    }

    // println!("{:#?}", vec[1]);


}

