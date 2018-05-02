use serializer::{Serializer, Serializable, FieldType};

pub struct RequestData {
    pub buildings: bool,
    pub provider: bool,
    pub map: bool,
    pub player: bool,
    pub hero: bool,
    pub abilities: bool,
    pub items: bool,
    pub draft: bool,
    pub wearables: bool,
}

impl Serializable for RequestData {
    fn serialize(&self) -> String {
        let mut serializer = Serializer { output: String::new(), indent: 2 };
        serializer.serialize_struct("data");
        if self.buildings { serializer.serialize_field("buildings", FieldType::FBool(self.buildings)); }
        if self.provider {serializer.serialize_field("provider", FieldType::FBool(self.provider)); }
        if self.map { serializer.serialize_field("map", FieldType::FBool(self.map)); }
        if self.player { serializer.serialize_field("player", FieldType::FBool(self.player)); }
        if self.hero { serializer.serialize_field("hero", FieldType::FBool(self.hero)); }
        if self.abilities { serializer.serialize_field("abilities", FieldType::FBool(self.abilities)); }
        if self.items { serializer.serialize_field("items", FieldType::FBool(self.items)); }
        if self.draft { serializer.serialize_field("draft", FieldType::FBool(self.draft)); }
        if self.wearables { serializer.serialize_field("wearables", FieldType::FBool(self.wearables)); }
        serializer.end(true)
    }
}