use solana_keypair::Signature;

pub struct SignatureModel {
    pub signature: Signature,
    pub valid_period: i64,
}
