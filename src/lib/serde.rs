trait Serde {
    fn serialize(&self) -> String;
    fn deserialize(s: String) -> Self;
}