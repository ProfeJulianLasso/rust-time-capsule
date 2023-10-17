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

pub fn proof(parameter: &mut ProofOfHistoryParams) -> ProofOfHistoryResult {
    validate_and_correct_default_parameters(parameter);

    // println!("Challenge: {:#?}", parameter);

    let mut result = ProofOfHistoryResult {
        solution: "".to_string(),
        proof: parameter.type_of_proof.as_ref().unwrap().to_string(),
        length: parameter.length.unwrap(),
        difficulty: parameter.difficulty.unwrap(),
    };

    if parameter.type_of_proof.as_ref().unwrap() == PROOF_OF_HISTORY_TYPE[0] {
        let pietrzak = PietrzakVDFParams(parameter.length.unwrap()).new();
        let challenge = pietrzak
            .solve(
                &parameter.challenge.as_bytes(),
                parameter.difficulty.unwrap(),
            )
            .unwrap();
        result.solution = hex::encode(&challenge);
    } else {
        let wesolowski = WesolowskiVDFParams(parameter.length.unwrap()).new();
        let challenge = wesolowski
            .solve(
                &parameter.challenge.as_bytes(),
                parameter.difficulty.unwrap(),
            )
            .unwrap();
        result.solution = hex::encode(&challenge);
    }
    result
}

pub fn verify(parameter: &mut ProofOfHistoryParams) -> String {
    validate_and_correct_default_parameters(parameter);
    validate_contents_of_alleged_solution(&parameter);

    // println!("Challenge: {:#?}", parameter);
    let alleged_solution = &hex::decode(parameter.alleged_solution.as_ref().unwrap().as_bytes());

    if parameter.type_of_proof.as_ref().unwrap() == PROOF_OF_HISTORY_TYPE[0] {
        let pietrzak = PietrzakVDFParams(parameter.length.unwrap()).new();
        match pietrzak.verify(
            &parameter.challenge.as_bytes(),
            parameter.difficulty.unwrap(),
            alleged_solution.as_ref().unwrap(),
        ) {
            Ok(()) => "Proof is valid".to_string(),
            Err(_) => "Invalid proof".to_string(),
        }
    } else {
        let wesolowski = WesolowskiVDFParams(parameter.length.unwrap()).new();
        match wesolowski.verify(
            &parameter.challenge.as_bytes(),
            parameter.difficulty.unwrap(),
            alleged_solution.as_ref().unwrap(),
        ) {
            Ok(()) => "Proof is valid".to_string(),
            Err(_) => "Invalid proof".to_string(),
        }
    }
}

fn valid_proof_type(proof: &String) {
    if !PROOF_OF_HISTORY_TYPE.contains(&proof.as_str()) {
        panic!("Invalid proof of history type");
    }
}

fn validate_and_correct_default_parameters(parameter: &mut ProofOfHistoryParams) {
    match parameter.type_of_proof.as_ref() {
        Some(proof) => valid_proof_type(&proof.to_string().to_lowercase()),
        None => parameter.type_of_proof = Some(PROOF_OF_HISTORY_TYPE_DEFAULT.to_string()),
    }

    match &parameter.length {
        Some(length) => {
            if *length < 1 {
                parameter.length = Some(LENGTH_DEFAULT);
            }
        }
        None => parameter.length = Some(LENGTH_DEFAULT),
    }

    match &parameter.difficulty {
        Some(difficulty) => {
            if *difficulty < DIFFICULTY_DEFAULT {
                parameter.difficulty = Some(DIFFICULTY_DEFAULT);
            }
        }
        None => parameter.difficulty = Some(DIFFICULTY_DEFAULT),
    }
}

fn validate_contents_of_alleged_solution(data: &ProofOfHistoryParams) {
    if data.alleged_solution.as_ref().unwrap().len() == 0 {
        panic!("Alleged solution is empty");
    }
}
