use crate::domain::Schedule;

pub trait Decoder {
    type Encoded;

    // Encode data, storing info needed to decode it.
    fn encode(&mut self, raw: &Schedule) -> Self::Encoded;
    fn decode(&self, encoded: &Self::Encoded) -> Schedule;
}