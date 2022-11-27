use std::fmt::Display;
use ibig::modular::ModuloRing;
use ibig::UBig;

pub struct RSAPublicKey {
    exponent: UBig,
    modulus: UBig,
}

impl RSAPublicKey {
    pub fn new(exponent: &[u8], modulus: &[u8]) -> Self {
        Self {
            exponent: UBig::from_be_bytes(exponent),
            modulus: UBig::from_be_bytes(modulus)
        }
    }

    pub fn check_signature(&self, message: &[u8], signature: &[u8]) -> bool {
        let modulo_ring = ModuloRing::new(&self.modulus);
        let signature = UBig::from_be_bytes(signature);
        let decrypted_signature = modulo_ring.from(&signature).pow(&self.exponent).residue().to_be_bytes();
        &decrypted_signature[decrypted_signature.len() - message.len()..] == message
    }
}

impl Display for RSAPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exponent: {:02X}, modulus: {:02X}", self.exponent, self.modulus)
    }
}