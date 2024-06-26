use crate::hash;
use starknet_crypto::FieldElement;
use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

/*
    Job Witness Object
    This object represents the output from the proving process.
    It holds a serialized proof as an array of FieldElement objects.
    This serialized proof can be deserialized into a StarkProof object by the verifier to proceed with the verification of the statement.
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JobWitness {
    pub proof: Vec<FieldElement>, // Serialized proof
}

impl Display for JobWitness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(hash!(self).to_be_bytes()))
    }
}
