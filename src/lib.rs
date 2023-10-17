mod blockchain;

use blockchain::proof_of_history::{proof, verify, ProofOfHistoryParams};
use neon::{
    prelude::{Context, FunctionContext, ModuleContext, Object},
    result::{JsResult, NeonResult},
    types::{JsObject, JsValue},
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

#[neon::main]
fn main(mut context: ModuleContext) -> NeonResult<()> {
    context.export_function("createProofOfHistory", create_proof_of_history)?;
    context.export_function("verifyProofOfHistory", verify_proof_of_history)?;
    Ok(())
}
