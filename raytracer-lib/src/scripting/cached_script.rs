use rhai::{Any, Engine, EvalAltResult};
use lib::{Colour, Intersection, Vec3d};
use scripting::engine;

use std::vec::Vec;

pub struct CachedScript {
    pub source: String,
}

pub struct Script {
    engine: Engine,
}

#[derive(Copy, Clone, Debug)]
pub enum FunctionCallError {
    FunctionNotFound,
    FunctionArgMismatch,
    WrongReturnType,
    UnexpectedError,
}

impl From<EvalAltResult> for FunctionCallError {
    fn from(err: EvalAltResult) -> Self {
        match err {
            EvalAltResult::ErrorFunctionNotFound => FunctionCallError::FunctionNotFound,
            EvalAltResult::ErrorFunctionArgMismatch => FunctionCallError::FunctionArgMismatch,
            _ => FunctionCallError::UnexpectedError,
        }
    }
}

impl Script {
    #![allow(dead_code)]

    pub fn new(script: &CachedScript) -> Self {
        let mut engine = engine::build();
        engine
            .consume(&script.source)
            .expect("Error while running script");

        Self { engine }
    }

    /* Methods from Raymarchable */
    pub fn call_distance(&self, mut point: Vec3d) -> Result<f64, FunctionCallError> {
        let ref result = *try!(
            self.engine
                .call_fn_raw("distance".to_string(), vec![&mut point])
        );

        if let Some(val) = result.downcast_ref::<f64>() {
            return Ok(*val);
        } else {
            return Err(FunctionCallError::WrongReturnType);
        }
    }
    pub fn call_normal(
        &self,
        mut point: Vec3d,
        mut direction: Vec3d,
    ) -> Result<Vec3d, FunctionCallError> {
        let ref result = *try!(
            self.engine
                .call_fn_raw("normal".to_string(), vec![&mut point, &mut direction])
        );

        if let Some(val) = result.downcast_ref::<Vec3d>() {
            return Ok(*val);
        } else {
            return Err(FunctionCallError::WrongReturnType);
        }
    }

    /* Methods for IFS */
    pub fn call_points(
        &self,
        mut point: Vec3d,
    ) -> Result<Box<Iterator<Item = Vec3d>>, FunctionCallError> {
        let ref result = *try!(
            self.engine
                .call_fn_raw("points".to_string(), vec![&mut point])
        );

        if let Some(ref retval) = result.downcast_ref::<Vec<Box<Any>>>() {
            let mut vec = Vec::new();

            for ref item in retval.iter() {
                if let Some(val) = item.downcast_ref::<Vec3d>() {
                    vec.push(*val);
                } else {
                    return Err(FunctionCallError::WrongReturnType);
                }
            }

            Ok(Box::new(vec.into_iter()))
        } else {
            Err(FunctionCallError::WrongReturnType)
        }
    }

    /* Methods from Material */
    pub fn call_base_colour(&self, mut isect: Intersection) -> Result<Colour, FunctionCallError> {
        let ref result = *try!(
            self.engine
                .call_fn_raw("base_colour".to_string(), vec![&mut isect])
        );

        if let Some(val) = result.downcast_ref::<Colour>() {
            return Ok(*val);
        } else {
            return Err(FunctionCallError::WrongReturnType);
        }
    }
    pub fn call_roughness(&self, mut isect: Intersection) -> Result<f32, FunctionCallError> {
        let ref result = *try!(
            self.engine
                .call_fn_raw("roughness".to_string(), vec![&mut isect])
        );

        if let Some(val) = result.downcast_ref::<f32>() {
            return Ok(*val);
        } else {
            return Err(FunctionCallError::WrongReturnType);
        }
    }
    pub fn call_reflectivity(&self, mut isect: Intersection) -> Result<f32, FunctionCallError> {
        let ref result = *try!(
            self.engine
                .call_fn_raw("reflectivity".to_string(), vec![&mut isect])
        );

        if let Some(val) = result.downcast_ref::<f32>() {
            return Ok(*val);
        } else {
            return Err(FunctionCallError::WrongReturnType);
        }
    }
}
