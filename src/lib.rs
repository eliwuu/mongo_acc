use futures::stream::StreamExt;
use js_sys::*;
use mongodb::bson;
use mongodb::{error, options::ClientOptions, options::Tls::Enabled, options::TlsOptions, Client};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

async fn get_client() -> Client {
    let uri = String::from("mongodb+srv://cluster0.izv2k.mongodb.net/myFirstDatabase?authSource=%24external&authMechanism=MONGODB-X509&retryWrites=true&w=majority");
    let app_name = "GeoApi";

    let cert_path: PathBuf = [r"cert.pem"].iter().collect();

    let tls_options = TlsOptions::builder();
    let opts = tls_options.cert_key_file_path(cert_path).build();

    let mut client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse URI");
    let enable_tls = Enabled(opts);
    client_options.tls = Some(enable_tls);
    client_options.app_name = Some(app_name.to_string());

    let client =
        mongodb::Client::with_options(client_options).expect("Could not connect to database");

    return client;
}

// pub async fn get_db(db_name: std::string::String) -> std::vec::Vec<error::Result<Document>> {
pub async fn get_collection(
    db_name: std::string::String,
    collection: std::string::String,
) -> mongodb::Collection<bson::Document> {
    let client = get_client().await;
    let db = client.database(db_name.as_str());

    let collection = db.collection::<bson::Document>(collection.as_str());

    return collection;
}

// #[wasm_bindgen]
// pub async fn get_all_docs(dbquery: DbQuery) -> Result<JsValue, JsValue> {
//     let client = get_client().await;
//     let db = client.database(dbquery.db_name.as_str());

//     let collection = db.collection::<bson::Document>("geojsons");

//     let get_all = collection
//         .find(None, None)
//         .await
//         .expect("Failed to get all documents");

//     let data: Vec<error::Result<bson::Document>> = get_all.collect().await;

//     let js_data = JsValue::from(data);

//     let promise = js_sys::Promise::resolve(js_data);
//     let result = wasm_bindgen_futures::JsFuture::from(promise).await;
//     Ok(result.unwrap())
// }

#[wasm_bindgen]
pub async fn get_all_docs(dbquery: DbQuery) -> Result<JsValue, JsValue> {
    let client = get_client().await;
    let db = client.database(dbquery.db_name.as_str());
    let collection: mongodb::Collection<GeoJSON> = db.collection("geojsons");

    let get_all = collection
        .find(None, None)
        .await
        .expect("Failed to get all documents");
    
    let data: Vec<error::Result<GeoJSON>> = get_all.collect().await;

    let deserialize = serde::derive_deserialize(&data);

    return Ok(JsValue::from_serde(data));
}

struct DbQuery {
    db_name: String,
    collection: String,
    filter: Option<String>,
}

// #[wasm_bindgen]
// pub async fn get_all(db: DbQuery) -> Result<JsValue, JsValue> {
//     let collection = get_collection(db.db_name, db.collection).await;
//     let collection = collection as mongodb::Collection<bson::Document>;
//     let get = collection
//         .find(None, None)
//         .await
//         .expect("Failed to get collection");

//     let data: Vec<Result<bson::Document, Error>> = get.collect().await.unwrap();

//     let promise = js_sys::Promise::resolve(get.into_iter().collect::<Vec<Document>>().into());
//     let result = wasm_bindgen_futures::JsFuture::from(promise).await;

//     return Ok(result);
// }

#[derive(Serialize, Deserialize)]
enum GeoType {
    Point,
    MultiPoint,
}

#[derive(Serialize, Deserialize)]
struct Geometry {
    r#type: String,
    coordinates: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
struct GeoJSON {
    r#type: GeoType,
    coordinates: Vec<f64>,
    data: AddressData,
}

#[derive(Serialize, Deserialize)]
struct AddressData {
    ip: Vec<String>,
    url: Option<Vec<String>>,
    misc: Option<String>,
}
