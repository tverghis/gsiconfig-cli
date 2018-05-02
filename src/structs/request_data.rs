use serializer::{Serializer, Serializable, FieldType};

/// Represents the data that the game sends to the listening endpoint.
///
/// Any fields that are set to `true` will be sent to the endpoint,
/// and are otherwise omitted.
///
/// Some more information can be found [here](https://github.com/antonpup/Dota2GSI#layout).
///
/// *Fields that are marked with **[?]** have best-guess documentation currently.*
pub struct RequestData {

    /// Information about building status. **[?]**
    pub buildings: bool,

    /// Some metadata about the sender.
    pub provider: bool,

    /// Information about the current map & game state.
    pub map: bool,

    /// Non-hero specific information about the player's current state.
    pub player: bool,

    /// Hero-specific information (like health and mana).
    pub hero: bool,

    /// Information about the state of the hero's spells.
    pub abilities: bool,

    /// Information about the state of the hero's items.
    pub items: bool,

    /// Information about the draft. **[?]**
    pub draft: bool,

    /// Information about the hero's currently equipped cosmetics. **[?]**
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