extern crate bellman;
extern crate pairing;
extern crate sapling_crypto;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use std::error::Error;

use wasm_bindgen::prelude::*;

use num_bigint::BigInt;
use num_traits::Num;

use bellman::{
    groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
        Parameters, Proof,
    },
    Circuit, ConstraintSystem, SynthesisError,
};

use ff::{BitIterator, PrimeField};
use pairing::bn256::{Bn256, Fr};
use rand::{ChaChaRng, SeedableRng};
use sapling_crypto::{
    babyjubjub::{
        edwards::Point, fs::Fs, FixedGenerators, JubjubBn256, JubjubEngine, JubjubParams,
    },
    circuit::{
        baby_ecc::fixed_base_multiplication,
        boolean::{AllocatedBit, Boolean},
    },
};

struct DiscreteLogCircuit<'a, E: JubjubEngine> {
    pub params: &'a E::Params,
    pub x: Option<E::Fr>,
}

impl<'a, E: JubjubEngine> Circuit<E> for DiscreteLogCircuit<'a, E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let mut x_bits = match self.x {
            Some(x) => BitIterator::new(x.into_repr()).collect::<Vec<_>>(),
            None => {
                vec![false; Fs::NUM_BITS as usize]
            }
        };

        x_bits.reverse();
        x_bits.truncate(Fs::NUM_BITS as usize);

        let x_bits = x_bits
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                AllocatedBit::alloc(cs.namespace(|| format!("scalar bit {}", i)), Some(b)).unwrap()
            })
            .map(|v| Boolean::from(v))
            .collect::<Vec<_>>();

        let h = fixed_base_multiplication(
            cs.namespace(|| "multiplication"),
            FixedGenerators::ProofGenerationKey,
            &x_bits,
            self.params,
        )?;

        h.inputize(cs)?;

        Ok(())
    }
}

#[derive(Serialize)]
pub struct KGGenerate {
    pub params: String,
}

#[derive(Serialize)]
pub struct KGProof {
    pub proof: String,
    pub h: String,
}

#[derive(Serialize)]
pub struct KGVerify {
    pub result: bool,
}

#[wasm_bindgen()]
pub fn generate(seed_slice: &[u32]) -> Result<JsValue, JsValue> {
    let res = run_generate(seed_slice);
    if res.is_ok() {
        Ok(JsValue::from_serde(&res.ok().unwrap()).unwrap())
    } else {
        Err(JsValue::from_str(&res.err().unwrap().to_string()))
    }
}

fn run_generate(seed_slice: &[u32]) -> Result<KGGenerate, Box<dyn Error>> {
    let rng = &mut ChaChaRng::from_seed(seed_slice);

    let j_params = &JubjubBn256::new();
    let params = generate_random_parameters::<Bn256, _, _>(
        DiscreteLogCircuit {
            params: j_params,
            x: None,
        },
        rng,
    )?;

    let mut v = vec![];

    params.write(&mut v)?;

    Ok(KGGenerate {
        params: hex::encode(&v[..]),
    })
}

#[wasm_bindgen()]
pub fn prove(seed_slice: &[u32], params: &str, x_hex: &str) -> Result<JsValue, JsValue> {
    let res = run_prove(seed_slice, params, x_hex);
    if res.is_ok() {
        Ok(JsValue::from_serde(&res.ok().unwrap()).unwrap())
    } else {
        Err(JsValue::from_str(&res.err().unwrap().to_string()))
    }
}

fn run_prove(seed_slice: &[u32], params: &str, x_hex: &str) -> Result<KGProof, Box<dyn Error>> {
    if params.len() == 0 {
        return Err("Params are empty. Did you generate or load params?".into());
    }
    let de_params = Parameters::<Bn256>::read(&hex::decode(params)?[..], true)?;

    let rng = &mut ChaChaRng::from_seed(seed_slice);
    let params = &JubjubBn256::new();

    let g = params.generator(FixedGenerators::ProofGenerationKey);
    let s = &format!("{}", Fs::char())[2..];
    let s_big = BigInt::from_str_radix(s, 16)?;
    let x_big = BigInt::from_str_radix(x_hex, 16)?;
    if x_big >= s_big {
        return Err(
            "x should be less than 60c89ce5c263405370a08b6d0302b0bab3eedb83920ee0a677297dc392126f1"
                .into(),
        );
    }
    let x_raw = &x_big.to_str_radix(10);
    let x = Fr::from_str(x_raw).ok_or("couldn't parse Fr")?;

    let xs = Fs::from_str(x_raw).ok_or("couldn't parse Fr")?;

    let h = g.mul(xs, params);

    let proof = create_random_proof(
        DiscreteLogCircuit {
            params: params,
            x: Some(x),
        },
        &de_params,
        rng,
    )?;

    let mut v = vec![];
    proof.write(&mut v)?;

    let mut v2 = vec![];
    h.write(&mut v2)?;

    Ok(KGProof {
        proof: hex::encode(&v[..]),
        h: hex::encode(&v2[..]),
    })
}

#[wasm_bindgen()]
pub fn verify(params: &str, proof: &str, h: &str) -> Result<JsValue, JsValue> {
    let res = run_verify(params, proof, h);
    if res.is_ok() {
        Ok(JsValue::from_serde(&res.ok().unwrap()).unwrap())
    } else {
        Err(JsValue::from_str(&res.err().unwrap().to_string()))
    }
}

fn run_verify(params: &str, proof: &str, h: &str) -> Result<KGVerify, Box<dyn Error>> {
    let j_params = &JubjubBn256::new();
    let de_params = Parameters::read(&hex::decode(params)?[..], true)?;
    let pvk = prepare_verifying_key::<Bn256>(&de_params.vk);
    let h = Point::<Bn256, _>::read(&hex::decode(h)?[..], j_params)?;

    let (h_x, h_y) = h.into_xy();
    let result = verify_proof(&pvk, &Proof::read(&hex::decode(proof)?[..])?, &[h_x, h_y])?;

    Ok(KGVerify { result: result })
}
