pub type SchemaGeometryVariant0 = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<f64>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<Vec<f64>>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant4 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<f64>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant5 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<Vec<f64>>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant6 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7ItemGeometriesVariant0 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7ItemGeometriesVariant1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<f64>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7ItemGeometriesVariant2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<Vec<f64>>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7ItemGeometriesVariant3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<f64>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7ItemGeometriesVariant4 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<Vec<f64>>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7ItemGeometriesVariant5 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaGeometryVariant7ItemGeometries {
    Variant0(SchemaGeometryVariant7ItemGeometriesVariant0),
    Variant1(SchemaGeometryVariant7ItemGeometriesVariant1),
    Variant2(SchemaGeometryVariant7ItemGeometriesVariant2),
    Variant3(SchemaGeometryVariant7ItemGeometriesVariant3),
    Variant4(SchemaGeometryVariant7ItemGeometriesVariant4),
    Variant5(SchemaGeometryVariant7ItemGeometriesVariant5),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchemaGeometryVariant7 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub geometries: Vec<SchemaGeometryVariant7ItemGeometries>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaGeometry {
    Variant0(SchemaGeometryVariant0),
    Variant1(SchemaGeometryVariant1),
    Variant2(SchemaGeometryVariant2),
    Variant3(SchemaGeometryVariant3),
    Variant4(SchemaGeometryVariant4),
    Variant5(SchemaGeometryVariant5),
    Variant6(SchemaGeometryVariant6),
    Variant7(SchemaGeometryVariant7),
}
pub type SchemaIdVariant0 = f64;
pub type SchemaIdVariant1 = String;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaId {
    Variant0(SchemaIdVariant0),
    Variant1(SchemaIdVariant1),
}
pub type SchemaPropertiesVariant0 = serde_json::Value;
pub type SchemaPropertiesVariant1 = ::std::collections::BTreeMap<String, serde_json::Value>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SchemaProperties {
    Variant0(SchemaPropertiesVariant0),
    Variant1(SchemaPropertiesVariant1),
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
    pub geometry: SchemaGeometry,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SchemaId>,
    pub properties: SchemaProperties,
    #[serde(rename = "type")]
    pub type_: String,
}
