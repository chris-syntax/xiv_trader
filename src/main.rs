use tungstenite::{connect, Message};
use url::Url;
use env_logger;
use bson::{doc, Document};
use bson::{bson, Bson};

fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("wss://universalis.app/api/ws").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let listings_add: Document = doc! {
        "event": "subscribe",
        "channel": "sales/add"
    };

    let mut listing_add_bin: Vec<u8> = vec![];
    listings_add.to_writer(&mut listing_add_bin).expect("Unable to serialize subscribe");

    socket.write_message(Message::Binary(listing_add_bin)).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let doc: Document = Document::from_reader(msg.into_data().as_slice()).unwrap();
        //let item = doc.get_i64("item").unwrap();
        //let prices = doc.get_array("sales").unwrap();
        //let mut new_prices: Vec<i64> = vec![];
        //for bson in prices.iter() {
        //    new_prices.push(bson.as_document().unwrap().get_i64("total").unwrap());
        //}


        //println!("Listing/Sale {} for {}", item, serde_json::to_string(&new_prices).unwrap());
        println!("Sales/Add {}", serde_json::to_string(&doc).unwrap());
    }
    
    // socket.close(None);
}