extern crate vdf;

use serde::{Deserialize, Serialize};
use vdf::{PietrzakVDFParams, VDFParams, WesolowskiVDFParams, VDF};

const PROOF_OF_HISTORY_TYPE_DEFAULT: &str = "wesolowski";
const PROOF_OF_HISTORY_TYPE: [&str; 2] = ["pietrzak", "wesolowski"];
const DIFFICULTY_DEFAULT: u64 = 200;

#[derive(Serialize, Debug, Deserialize)]
pub struct ProofOfHistory {
    pub challenge: String,
    pub difficulty: Option<u64>,
    pub alleged_solution: Option<String>,
    pub type_of_proof: Option<String>,
}

pub fn proof(data: &mut ProofOfHistory) -> String {
    match data.type_of_proof.as_ref() {
        Some(proof) => {
            valid_proof_type(&proof.to_string().to_lowercase());
        }
        None => {
            data.type_of_proof = Some(PROOF_OF_HISTORY_TYPE_DEFAULT.to_string());
        }
    }

    match &data.difficulty {
        Some(difficulty) => {
            if *difficulty < 66 {
                panic!("Difficulty must be greater than or equal to 66");
            }
        }
        None => {
            data.difficulty = Some(DIFFICULTY_DEFAULT);
        }
    }

    println!("Challenge: {:#?}", data);

    if data.type_of_proof.as_ref().unwrap() == PROOF_OF_HISTORY_TYPE[0] {
        let pietrzak = PietrzakVDFParams(2048).new();
        let challenge = pietrzak
            .solve(&data.challenge.as_bytes(), data.difficulty.unwrap())
            .unwrap();
        hex::encode(&challenge)
    } else {
        let wesolowski = WesolowskiVDFParams(2048).new();
        let challenge = wesolowski
            .solve(&data.challenge.as_bytes(), data.difficulty.unwrap())
            .unwrap();
        hex::encode(&challenge)
    }
}

fn valid_proof_type(proof: &String) {
    if !PROOF_OF_HISTORY_TYPE.contains(&proof.as_str()) {
        panic!("Invalid proof of history type");
    }
}
