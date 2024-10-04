use std::collections::BTreeMap;

#[derive(Clone, PartialEq)]
#[allow(dead_code)] //TODO: Remove
pub enum DataType {
    Integer(i32),
    Text(String),
    Float(f32),
}

#[allow(dead_code)] //TODO: Remove
pub enum DataKind {
    Integer,
    Text,
    Float,
}

#[derive(Clone, PartialEq)]
pub struct ColumnSchema {
    pub field_name: String,
    pub data_type: DataType,
}

#[allow(dead_code)] //TODO: Remove
pub struct Row {
    pub values: Vec<DataType>,
}

#[allow(dead_code)] //TODO: Remove
struct Table<K> {
    pub name: String,
    pub schema: Vec<ColumnSchema>,
    pub data: BTreeMap<K, Row>,
}

#[allow(dead_code)] //TODO: Remove
impl<K: Ord> Table<K> {
    pub fn new(name: &str, schema: Vec<ColumnSchema>) -> Self {
        Table {
            name: String::from(name),
            schema,
            data: BTreeMap::new(),
        }
    }

    /// Inserts a new field to the table, also checks if another field with the
    /// same name exists, in which case it doesn´t add the field
    pub fn insert_field(&mut self, name: &str, field_type: DataType) {
        if !self.schema.contains(&ColumnSchema {
            field_name: String::from(name),
            data_type: field_type.clone(),
        }) {
            self.schema.push(ColumnSchema {
                field_name: String::from(name),
                data_type: field_type,
            })
        }
    }

    /// Note that this method also updates data in the situation where
    /// key is already found in BTreeMap
    pub fn insert_data(&mut self, key: K, data: Row) {
        self.data.insert(key, data);
    }
}
