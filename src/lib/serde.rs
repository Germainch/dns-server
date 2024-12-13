use bytes::Bytes;

pub(crate) trait DNSSerialization {
    fn serialize(&self) -> Bytes;
    fn deserialize(s: Bytes) -> Self;
}
