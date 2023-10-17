extern crate vdf;

use serde::{Deserialize, Serialize};
use vdf::{PietrzakVDFParams, VDFParams, WesolowskiVDFParams, VDF};

const PROOF_OF_HISTORY_TYPE_DEFAULT: &str = "wesolowski";
const PROOF_OF_HISTORY_TYPE: [&str; 2] = ["pietrzak", "wesolowski"];
const DIFFICULTY_DEFAULT: u64 = 66;
const LENGTH_DEFAULT: u16 = 2048;

#[derive(Serialize, Debug, Deserialize)]
pub struct ProofOfHistoryParams {
    pub challenge: String,
    pub length: Option<u16>,
    pub difficulty: Option<u64>,
    pub alleged_solution: Option<String>,
    pub type_of_proof: Option<String>,
}

pub struct ProofOfHistoryResult {
    pub solution: String,
    pub proof: String,
    pub length: u16,
    pub difficulty: u64,
}

pub fn proof(data: &mut ProofOfHistoryParams) -> ProofOfHistoryResult {
    match data.type_of_proof.as_ref() {
        Some(proof) => valid_proof_type(&proof.to_string().to_lowercase()),
        None => data.type_of_proof = Some(PROOF_OF_HISTORY_TYPE_DEFAULT.to_string()),
    }

    match &data.length {
        Some(length) => {
            if *length < 1 {
                data.length = Some(LENGTH_DEFAULT);
            }
        }
        None => data.length = Some(LENGTH_DEFAULT),
    }

    match &data.difficulty {
        Some(difficulty) => {
            if *difficulty < DIFFICULTY_DEFAULT {
                data.difficulty = Some(DIFFICULTY_DEFAULT);
            }
        }
        None => data.difficulty = Some(DIFFICULTY_DEFAULT),
    }

    // println!("Challenge: {:#?}", data);

    let mut result = ProofOfHistoryResult {
        solution: "".to_string(),
        proof: data.type_of_proof.as_ref().unwrap().to_string(),
        length: data.length.unwrap(),
        difficulty: data.difficulty.unwrap(),
    };

    if data.type_of_proof.as_ref().unwrap() == PROOF_OF_HISTORY_TYPE[0] {
        let pietrzak = PietrzakVDFParams(data.length.unwrap()).new();
        let challenge = pietrzak
            .solve(&data.challenge.as_bytes(), data.difficulty.unwrap())
            .unwrap();
        result.solution = hex::encode(&challenge);
    } else {
        let wesolowski = WesolowskiVDFParams(data.length.unwrap()).new();
        let challenge = wesolowski
            .solve(&data.challenge.as_bytes(), data.difficulty.unwrap())
            .unwrap();
        result.solution = hex::encode(&challenge);
    }
    result
}

fn valid_proof_type(proof: &String) {
    if !PROOF_OF_HISTORY_TYPE.contains(&proof.as_str()) {
        panic!("Invalid proof of history type");
    }
}
