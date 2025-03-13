pub mod rsa {
    use num_bigint::BigInt;
    use constants::rsa::title_protocol::{SERVER_EXPONENT, SERVER_MODULUS};
    use crate::packet::Packet;
    
    pub fn decrypt_rsa_block(mut packet: Packet, packet_length: usize) -> Packet {
        let rsa_bytes_vec = packet.gbytes(packet_length);
        let rsa_bytes = BigInt::from_bytes_be(num_bigint::Sign::Plus, &rsa_bytes_vec);
        let decrypted_byes = rsa_bytes.modpow(&*SERVER_EXPONENT, &*SERVER_MODULUS);
        let (_, bytes) = decrypted_byes.to_bytes_be();
        
        Packet::from(bytes)
    }
}