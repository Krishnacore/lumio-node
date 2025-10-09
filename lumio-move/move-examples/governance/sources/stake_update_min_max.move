script {
    use lumio_framework::lumio_governance;
    use lumio_framework::coin;
    use lumio_framework::lumio_coin::LumioCoin;
    use lumio_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = lumio_governance::resolve(proposal_id, @lumio_framework);
        let one_lumio_coin_with_decimals = 10 ** (coin::decimals<LumioCoin>() as u64);
        // Change min to 1000 and max to 1M Lumio coins.
        let new_min_stake = 1000 * one_lumio_coin_with_decimals;
        let new_max_stake = 1000000 * one_lumio_coin_with_decimals;
        staking_config::update_required_stake(&framework_signer, new_min_stake, new_max_stake);
    }
}
