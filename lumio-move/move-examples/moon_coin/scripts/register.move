//:!:>moon
script {
    fun register(account: &signer) {
        lumio_framework::managed_coin::register<MoonCoin::moon_coin::MoonCoin>(account)
    }
}
//<:!:moon
