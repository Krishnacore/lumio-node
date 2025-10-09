module poc::scalar_from_sha512_internal {
    use lumio_std::ristretto255;
    use lumio_std::lumio_hash;

    public entry fun main(_owner:&signer) {
        let input = b"hello world";
        let hash_digest = lumio_hash::sha2_512(input);
        let _scalar = ristretto255::new_scalar_from_sha512(hash_digest);
    }

    #[test(owner=@0x123)]
    fun a(owner:&signer){
       main(owner);
    }
}
