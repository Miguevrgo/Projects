use std::collections::BTreeMap;

pub enum DataType {
    Integer(i32),
    Text(String),
    Float(f32),
}

pub enum DataKind {
    Integer,
    Text,
    Float,
}

pub struct ColumnSchema {
    pub field_name: String,
    pub data_type: DataType,
}

pub struct Row {
    pub values: Vec<DataType>,
}

struct Table<K> {
    pub name: String,
    pub schema: Vec<ColumnSchema>,
    pub data: BTreeMap<K, Row>,
}

impl<K: Ord> Table<K> {
    pub fn new(name: &str, schema: Vec<ColumnSchema>) -> Self {
        Table {
            name: String::from(name),
            schema,
            data: BTreeMap::new(),
        }
    }

    pub fn insert_field(name: &str, content: DataType) {}
}
