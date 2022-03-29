pub(crate) mod geo_json;

use crate::geo_json::GeoJSON;
use futures::stream::StreamExt;
use mongodb::bson;
use mongodb::{ error, options::ClientOptions, options::Tls::Enabled, options::TlsOptions, Client};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

async fn get_client() -> Client {
    let uri = String::from("mongodb+srv://cluster0.izv2k.mongodb.net/myFirstDatabase?authSource=%24external&authMechanism=MONGODB-X509&retryWrites=true&w=majority");
    let app_name = "GeoApi";

    let cert_path: PathBuf = PathBuf::from("cert.pem");

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

pub async fn get_collection(
    db_name: std::string::String,
    collection: std::string::String,
) -> mongodb::Collection<bson::Document> {
    let client = get_client().await;
    let db = client.database(db_name.as_str());

    let collection = db.collection::<bson::Document>(collection.as_str());

    return collection;
}

#[wasm_bindgen]
pub async fn get_all_docs(db_name: String, db_collection: String) -> Result<JsValue, JsValue> {
    let client = get_client().await;
    let db = client.database(db_name.as_str());
    let collection: mongodb::Collection<GeoJSON> = db.collection(db_collection.as_str());

    let get_all = collection
        .find(None, None)
        .await
        .expect("Failed to get all documents");
    
    let data= get_all.collect::<Vec<error::Result<GeoJSON>>>().await;

    let ls = data.into_iter().map(|x| x.expect("no data")).collect::<Vec<GeoJSON>>();

    let deserialize = serde_json::to_string(&ls).unwrap();

    let js_value = JsValue::from_str(deserialize.as_str());

    let promise = js_sys::Promise::resolve(&js_value);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;

    return result;
}