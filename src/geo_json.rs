use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum GeoType {
    Point,
    MultiPoint,
}

#[derive(Serialize, Deserialize)]
pub struct Geometry {
    r#type: String,
    coordinates: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct GeoJSON {
    r#type: GeoType,
    coordinates: Vec<f64>,
    data: AddressData,
}

#[derive(Serialize, Deserialize)]
pub struct AddressData {
    ip: Vec<String>,
    url: Option<Vec<String>>,
    misc: Option<String>,
}
