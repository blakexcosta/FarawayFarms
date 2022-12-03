    
use mongodb::{bson::doc, options::ClientOptions, Client};
// This trait is required to use `try_next()` on the cursor
use futures::{stream::TryStreamExt, io};
use mongodb::bson::Document;
use mongodb::options::FindOptions;


// connect to a mongo instance
pub async fn connect() -> mongodb::error::Result<()> {
    //---------------------------------------------------------------------
    // DATABASE STUFF
    // mongodb://root:mypassword@localhost:27017/?authSource=admin&readPreference=primary&appname=MongoDB%20Compass&ssl=false
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://root:mypassword@localhost:27017/?authSource=admin&readPreference=primary&appname=MongoDB%20Compass&ssl=false")
            .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Books App".to_string()); // set an app name

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    println!("\nCollection names in blake_test_db");
    // Get a handle to a database.
    let db = client.database("player_market_db");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    //---------------------------------------------------------------------
    // Insert books into a database
    let typed_collection: mongodb::Collection<Document> = db.collection("player_market_collection");
    // // insert some orders
    // let docs = vec![
    //     doc! { "item_name": "soup_can", "quantity": 2, "price": 1000, "posted_by": "Blake Costa", "player_note": "Deez some cans", "posting_type": "sell", "posted_date": "2022-12-01", "days_active": 7 },
    //     doc! { "item_name": "soup_can", "quantity": 1, "price": 500, "posted_by": "Shawna Costa", "player_note": "I need some cans", "posting_type": "buy", "posted_date": "2022-12-01", "days_active": 7 },
    //     doc! { "item_name": "scrap", "quantity": 1, "price": 1500, "posted_by": "Rick Olback", "player_note": "Selling some stuff", "posting_type": "sell", "posted_date": "2022-12-01", "days_active": 7 },
    // ];
    // // Insert some documents into the "mydb.books" collection.
    // typed_collection.insert_many(docs, None).await?;

    //---------------------------------------------------------------------
    //get a result from the database
    // Query the books in the collection with a filter and an option.
    let filter = doc! {}; // get a specific item doc! { "title": "Iliad" }
    let find_options = FindOptions::builder().sort(doc! { "_id": 1 }).build(); // this just means to sort the filter by title in ascending order, -1 for descending order
    let mut cursor = typed_collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        println!("book: {}", book);
    }
    Ok(())
}
    