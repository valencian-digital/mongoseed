mod generation;
use bson::doc;
use std::time::Instant;

fn user_generator() -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Jane Doe"
    };
}

fn post_generator() -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "text": "Hello World"
    };
}

fn main() {
    let amount = 10000;
    let now = Instant::now();
    let collections = vec![String::from("users"), String::from("posts")];
    let entity_generators: Vec<generation::EntityGenerator> = vec![user_generator, post_generator];
    generation::create_data(collections, entity_generators, amount);

    println!("{:?}", now.elapsed());
}
