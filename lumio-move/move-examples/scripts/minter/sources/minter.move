script {
    use std::signer;
    use lumio_framework::lumio_account;
    use lumio_framework::lumio_coin;
    use lumio_framework::coin;

    // Tune this parameter based upon the actual gas costs
    const GAS_BUFFER: u64 = 100000;
    const U64_MAX: u64 = 18446744073709551615;

    fun main(minter: &signer, dst_addr: address, amount: u64) {
        let minter_addr = signer::address_of(minter);

        // Do not mint if it would exceed U64_MAX
        let balance = coin::balance<lumio_coin::LumioCoin>(minter_addr);
        if (balance < U64_MAX - amount - GAS_BUFFER) {
            lumio_coin::mint(minter, minter_addr, amount + GAS_BUFFER);
        };

        lumio_account::transfer(minter, dst_addr, amount);
    }
}
