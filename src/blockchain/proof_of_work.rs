use blake3::hash as blake3;
use easy_parallel::Parallel;
use regex::Regex;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

use libsodium_sys::randombytes_random;

static GLOBAL_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct ProofOfWork {
    pub challenge: String,
    pub difficulty: u8,
    pub stress: u8,
}

pub struct CheckProofOfWork {
    pub challenge: String,
    pub nonce: String,
    pub hash: String,
}

pub struct ResultProofOfWork {
    pub cores: u8,
    pub stress: u8,
    pub difficulty: u8,
    pub nonce: String,
    pub hash: String,
    pub time_elapsed: u64,
}

pub fn create_block(parameter: &ProofOfWork) -> ResultProofOfWork {
    let start = Instant::now();
    let challenge = &parameter.challenge;
    let difficulty = &parameter.difficulty;
    let stress = &parameter.stress;

    let nonce = Arc::new(Mutex::new(String::new()));
    let data = Arc::new(Mutex::new(String::new()));
    let hash = Arc::new(Mutex::new(String::new()));

    let mut cores: u8 = (((num_cpus::get() * *stress as usize) / 100) as f32).ceil() as u8;
    if cores == 0 {
        cores = 1;
    } else if cores > num_cpus::get() as u8 {
        cores = num_cpus::get() as u8;
    }

    // println!("Params: {:#?}", parameter);

    Parallel::new()
        .each(0..cores, |_index| {
            create(&challenge, &difficulty.to_string(), &nonce, &data, &hash)
        })
        .run();
    let duration = start.elapsed();

    let result = ResultProofOfWork {
        cores: cores,
        stress: *stress,
        difficulty: *difficulty,
        nonce: nonce.lock().unwrap().to_string(),
        hash: hash.lock().unwrap().to_string(),
        time_elapsed: duration.as_secs(),
    };
    result
}

pub fn check_block(parameter: &CheckProofOfWork) -> String {
    check(&parameter.challenge, &parameter.nonce, &parameter.hash)
}

fn check(challenge: &String, nonce: &String, hash: &String) -> String {
    let data = challenge.replace(
        "\"nonce\":null",
        &String::from("\"nonce\": ".to_owned() + &nonce),
    );
    let hash_confirm = blake3(&data.as_bytes());
    if hash_confirm.to_string().eq(hash) {
        "OK".to_string()
    } else {
        "ERROR".to_string()
    }
}

fn create(
    info: &String,
    level: &String,
    nonce_final: &Arc<Mutex<String>>,
    data_final: &Arc<Mutex<String>>,
    hash_final: &Arc<Mutex<String>>,
) {
    let mut nonce = nonce_random().to_string();
    let mut data = info.replace(
        "\"nonce\":null",
        &String::from("\"nonce\": ".to_owned() + &nonce),
    );
    let mut hash = blake3(&data.as_bytes());

    let pattern = format!(r"^(0){{{}}}", level);
    let regex = Regex::new(pattern.as_str()).unwrap();
    let mut option = regex.is_match(&hash.to_string());

    if option == true {
        set_flag_to_true();
        *nonce_final.lock().unwrap() = nonce;
        *data_final.lock().unwrap() = data;
        *hash_final.lock().unwrap() = hash.to_string();
        return;
    }

    while option == false && get_flag() == false {
        nonce = nonce_random().to_string();
        data = info.replace(
            "\"nonce\":null",
            &String::from("\"nonce\": ".to_owned() + &nonce),
        );
        hash = blake3(&data.as_bytes());
        option = regex.is_match(&hash.to_string());

        if option == true {
            set_flag_to_true();
            *nonce_final.lock().unwrap() = nonce;
            *data_final.lock().unwrap() = data;
            *hash_final.lock().unwrap() = hash.to_string();
            return;
        }
    }
}

fn nonce_random() -> u32 {
    unsafe {
        return randombytes_random();
    }
}

fn set_flag_to_true() {
    GLOBAL_FLAG.store(true, Ordering::SeqCst);
}

fn get_flag() -> bool {
    return GLOBAL_FLAG.load(Ordering::SeqCst);
}
