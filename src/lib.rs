pub mod kernel;
pub mod pqc;

pub fn ntos_status() -> &'static str {
    "NTOS Red Team v1.0 - QEMU Bootable [40 files]"
}

#[cfg(test)]
mod tests {
    use super::pqc::*;
    
    #[test]
    fn test_qrap_keygen() {
        let seed = b"classical_seed";
        let qnoise = b"q_entropy_32bytes!!";
        let instance = "saksi-monad-001";
        
        let (pk, sk) = generate_qrap_seeded_keypair(seed, qnoise, instance).unwrap();
        assert_eq!(pk.len(), 32);
        assert_eq!(sk.len(), 32);
        println!("QRAP Keys: {} bytes", pk.len());
    }
}
