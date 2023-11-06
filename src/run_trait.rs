// learn about Rust's traits
// Code along from: https://archive.ph/jpRoE

// define struct. similar to typescript interface
struct Document {
    id: String,
    timestamp: u64,
    revised: bool,
}
struct Image {
    id: String,
    // timestamp: u64,
    // mime_type: String,
}

// simple implementation of a get document/image function
// fn get_document_v1(id: String, documents: Vec<Document>) -> Option<Document> {
//     documents.into_iter().find(|document| document.id == id)
// }
// fn get_image_v1(id: String, images: Vec<Image>) -> Option<Image> {
//     images.into_iter().find(|image| image.id == id)
// }

// both image and document share a common characteristic: they have an id that can be compared
trait Compare {
    fn compare(&self, id: &str) -> bool;
}
impl Compare for Document {
    fn compare(&self, id: &str) -> bool {
        self.id == id
    }
}
impl Compare for Image {
    fn compare(&self, id: &str) -> bool {
        self.id == id
    }
}

// implement a generic get function using the Compare trait
fn get<T: Compare>(id: String, elements: Vec<T>) -> Option<T> {
    elements.into_iter().find(|element| element.compare(&id))
}
fn get_document_v2(id: String, documents: Vec<Document>) -> Option<Document> {
    get(id, documents)
}
// fn get_image_v2(id: String, images: Vec<Image>) -> Option<Image> {
//     get(id, images)
// }

// attempt to run the code
pub fn run_trait() {
    // create dummy data
    let docs = vec![
        Document {
            id: "1".to_string(),
            timestamp: 0,
            revised: false,
        },
        Document {
            id: "2".to_string(),
            timestamp: 0,
            revised: true,
        },
    ];

    let result = get_document_v2("2".to_string(), docs);

    if let Some(doc) = result {
        println!(
            "Found document id: {}, timestamp: {}, revised: {}",
            doc.id, doc.timestamp, doc.revised
        );
    } else {
        println!("Document not found");
    }
}
