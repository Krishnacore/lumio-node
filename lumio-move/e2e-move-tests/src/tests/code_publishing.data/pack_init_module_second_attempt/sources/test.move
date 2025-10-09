module 0xcafe::test {
    use lumio_framework::coin::{Self, Coin};
    use lumio_framework::lumio_coin::LumioCoin;

    struct State has key {
        important_value: u64,
        coins: Coin<LumioCoin>,
    }

    fun init_module(s: &signer) {
        move_to(s, State {
            important_value: get_value(),
            coins: coin::zero<LumioCoin>(),
        })
    }

    fun get_value(): u64 {
        2
    }
}
