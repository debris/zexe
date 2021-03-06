#![deny(
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]
#![deny(
    unused_qualifications,
    variant_size_differences,
    stable_features,
    unreachable_pub
)]
#![deny(
    non_shorthand_field_patterns,
    unused_attributes,
    unused_imports,
    unused_extern_crates
)]
#![deny(
    renamed_and_removed_lints,
    stable_features,
    unused_allocation,
    unused_comparisons,
    bare_trait_objects
)]
#![deny(
    const_err,
    unused_must_use,
    unused_mut,
    unused_unsafe,
    private_in_public,
    unsafe_code
)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate derivative;

#[cfg_attr(test, macro_use)]
pub mod bytes;
pub use self::bytes::*;

#[macro_use]
pub mod serialize;
pub use self::serialize::*;

pub mod biginteger;
pub use self::biginteger::*;

pub mod curves;
pub use self::curves::*;

#[macro_use]
pub mod fields;
pub use self::fields::*;

pub mod groups;
pub use self::groups::*;

mod rand;
pub use self::rand::*;


mod to_field_vec;
pub use to_field_vec::ToConstraintField;

pub mod msm;
pub use self::msm::*;

pub use num_traits::{One, Zero};

pub mod prelude {
    pub use crate::biginteger::BigInteger;

    pub use crate::fields::{Field, PrimeField, SquareRootField, FpParameters};

    pub use crate::groups::Group;

    pub use crate::curves::{ProjectiveCurve, AffineCurve, PairingCurve, PairingEngine};

    pub use crate::rand::UniformRand;

    pub use num_traits::{One, Zero};
}
