script {
    use lumio_framework::lumio_governance;
    use lumio_framework::staking_config;

    fun main(proposal_id: u64) {
        let framework_signer = lumio_governance::resolve(proposal_id, @lumio_framework);
        // Update voting power increase limit to 10%.
        staking_config::update_voting_power_increase_limit(&framework_signer, 10);
    }
}
