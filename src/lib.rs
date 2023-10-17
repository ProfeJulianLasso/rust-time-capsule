mod blockchain;

use blockchain::proof_of_history::{proof, ProofOfHistory};
use neon::{
    prelude::{Context, FunctionContext, ModuleContext, Object},
    result::{JsResult, NeonResult},
    types::{JsObject, JsString, JsValue},
};

fn create_proof_of_history(mut context: FunctionContext) -> JsResult<JsObject> {
    let params = context
        .argument::<JsValue>(0)
        .unwrap_or_else(|_| panic!("Invalid data"));

    let mut params: ProofOfHistory = match neon_serde3::from_value(&mut context, params) {
        Ok(value) => value,
        Err(error) => {
            return context.throw_error(error.to_string());
        }
    };

    let result = proof(&mut params);

    let object = context.empty_object();
    let result = context.string(&result);
    object.set(&mut context, "result", result)?;

    Ok(object)
}

fn verify_proof_of_history(mut context: FunctionContext) -> JsResult<JsString> {
    Ok(context.string("hello node"))
}

#[neon::main]
fn main(mut context: ModuleContext) -> NeonResult<()> {
    context.export_function("createProofOfHistory", create_proof_of_history)?;
    context.export_function("verify_proof_of_history", verify_proof_of_history)?;
    Ok(())
}
