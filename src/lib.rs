extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate cardgame_macros;
mod real_decision_maker;
pub use real_decision_maker::RealDecisionMaker;
#[allow(non_snake_case)]
pub mod json_gen;
