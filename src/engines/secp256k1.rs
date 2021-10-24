use hex_literal::hex;

use k256::elliptic_curve::group::ff::PrimeField;
use k256::elliptic_curve::group::GroupEncoding;
use k256::{elliptic_curve::generic_array::GenericArray, Scalar, ProjectivePoint};

use crate::engines::{BasepointProvider, ff_group::{FfGroupConversions, FfGroupEngine}};

pub struct Secp256k1Conversions;
impl FfGroupConversions for Secp256k1Conversions {
  type Scalar = Scalar;
  type Point = ProjectivePoint;

  fn scalar_from_bytes_mod(scalar: [u8; 32]) -> Self::Scalar {
    Scalar::from_bytes_reduced(&scalar.into())
  }

  fn little_endian_bytes_to_scalar(bytes: [u8; 32]) -> anyhow::Result<Self::Scalar> {
    let mut bytes = bytes;
    bytes.reverse();
    Scalar::from_repr(bytes.into()).ok_or(anyhow::anyhow!("Invalid scalar"))
  }

  fn scalar_to_bytes(scalar: &Self::Scalar) -> [u8; 32] {
    scalar.to_bytes().into()
  }

  fn point_to_bytes(point: &Self::Point) -> Vec<u8> {
     point.to_bytes().as_ref().to_vec()
  }
}

pub struct Secp256k1Basepoints;
impl BasepointProvider for Secp256k1Basepoints {
  type Point = ProjectivePoint;

  fn basepoint() -> Self::Point {
    ProjectivePoint::generator()
  }

  fn alt_basepoint() -> Self::Point {
    // Taken from Grin: https://github.com/mimblewimble/rust-secp256k1-zkp/blob/ed4297b0e3dba9b0793aab340c7c81cda6460bcf/src/constants.rs#L97
    // See comments on the Ed25519 engine about pub status
    ProjectivePoint::from_bytes(
      &GenericArray::from_slice(&hex!("0250929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0"))
    ).unwrap()
  }
}

pub type Secp256k1Engine = FfGroupEngine<Scalar, ProjectivePoint, Secp256k1Conversions, Secp256k1Basepoints>;
