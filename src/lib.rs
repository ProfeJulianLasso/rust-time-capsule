mod blockchain;

use blockchain::{
    proof_of_history::{proof, verify, ProofOfHistoryParams},
    proof_of_work::{check_block, create_block, CheckProofOfWork, ProofOfWork},
};
use neon::{
    prelude::{Context, FunctionContext, ModuleContext, Object},
    result::{JsResult, NeonResult},
    types::{JsNumber, JsObject, JsString, JsValue},
};

fn create_proof_of_history(mut context: FunctionContext) -> JsResult<JsObject> {
    let params = context
        .argument::<JsValue>(0)
        .unwrap_or_else(|_| panic!("Invalid data"));

    let mut params: ProofOfHistoryParams = match neon_serde3::from_value(&mut context, params) {
        Ok(value) => value,
        Err(error) => {
            return context.throw_error(error.to_string());
        }
    };

    let result = proof(&mut params);

    let object = context.empty_object();

    let difficulty = context.number(result.difficulty as f64);
    object.set(&mut context, "difficulty", difficulty)?;

    let length = context.number(result.length);
    object.set(&mut context, "length", length)?;

    let proof = context.string(&result.proof);
    object.set(&mut context, "proof", proof)?;

    let solution = context.string(&result.solution);
    object.set(&mut context, "solution", solution)?;

    Ok(object)
}

fn verify_proof_of_history(mut context: FunctionContext) -> JsResult<JsObject> {
    let params = context
        .argument::<JsValue>(0)
        .unwrap_or_else(|_| panic!("Invalid data"));

    let mut params: ProofOfHistoryParams = match neon_serde3::from_value(&mut context, params) {
        Ok(value) => value,
        Err(error) => {
            return context.throw_error(error.to_string());
        }
    };

    let result = verify(&mut params);

    let object = context.empty_object();
    let result = context.string(&result);
    object.set(&mut context, "result", result)?;

    Ok(object)
}

fn create_block_proof_of_work(mut context: FunctionContext) -> JsResult<JsObject> {
    let challenge = context
        .argument::<JsString>(0)
        .unwrap_or_else(|_| panic!("Invalid data"));
    let difficulty = context
        .argument::<JsNumber>(1)
        .unwrap_or_else(|_| panic!("Invalid data"));
    let stress = context
        .argument::<JsNumber>(2)
        .unwrap_or_else(|_| panic!("Invalid data"));

    let parameters = ProofOfWork {
        challenge: challenge.value(&mut context),
        difficulty: difficulty.value(&mut context) as u8,
        stress: stress.value(&mut context) as u8,
    };
    let result = create_block(&parameters);

    let object = context.empty_object();

    let cores = context.number(result.cores as f64);
    object.set(&mut context, "cores", cores)?;

    let stress = context.number(result.stress as f64);
    object.set(&mut context, "stress", stress)?;

    let difficulty = context.number(result.difficulty as f64);
    object.set(&mut context, "difficulty", difficulty)?;

    let nonce = context.string(result.nonce);
    object.set(&mut context, "nonce", nonce)?;

    let hash = context.string(result.hash);
    object.set(&mut context, "hash", hash)?;

    let time_elapsed = context.number(result.time_elapsed as f64);
    object.set(&mut context, "time_elapsed", time_elapsed)?;

    Ok(object)
}

fn check_block_proof_of_work(mut context: FunctionContext) -> JsResult<JsObject> {
    let challenge = context
        .argument::<JsString>(0)
        .unwrap_or_else(|_| panic!("Invalid data"));
    let nonce = context
        .argument::<JsString>(1)
        .unwrap_or_else(|_| panic!("Invalid data"));
    let hash = context
        .argument::<JsString>(2)
        .unwrap_or_else(|_| panic!("Invalid data"));

    let parameters = CheckProofOfWork {
        challenge: challenge.value(&mut context),
        nonce: nonce.value(&mut context),
        hash: hash.value(&mut context),
    };
    let result = check_block(&parameters);

    let object = context.empty_object();
    let result = context.string(&result);
    object.set(&mut context, "result", result)?;

    Ok(object)
}

#[neon::main]
fn main(mut context: ModuleContext) -> NeonResult<()> {
    context.export_function("createProofOfHistory", create_proof_of_history)?;
    context.export_function("verifyProofOfHistory", verify_proof_of_history)?;
    context.export_function("createBlockProofOfWork", create_block_proof_of_work)?;
    context.export_function("verifyBlockProofOfWork", check_block_proof_of_work)?;
    Ok(())
}
