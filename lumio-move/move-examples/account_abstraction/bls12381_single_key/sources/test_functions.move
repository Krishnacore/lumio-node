module aa::test_functions {
    use lumio_framework::lumio_account;

    /// test function for multi-agent aa.
    public entry fun transfer_to_the_last(a: &signer, b: &signer, c: &signer, d: address) {
        lumio_account::transfer(a, d, 1);
        lumio_account::transfer(b, d, 1);
        lumio_account::transfer(c, d, 1);
    }
}
