// Initialize on-chain randomness resources.
script {
    use lumio_framework::lumio_governance;
    use lumio_framework::config_buffer;
    use lumio_framework::dkg;
    use lumio_framework::randomness;
    use lumio_framework::randomness_config;
    use lumio_framework::reconfiguration_state;

    fun main(proposal_id: u64) {
        let framework = lumio_governance::resolve_multi_step_proposal(
            proposal_id,
            @0x1,
            {{ script_hash }},
        );
        config_buffer::initialize(&framework); // on-chain config buffer
        dkg::initialize(&framework); // DKG state holder
        reconfiguration_state::initialize(&framework); // reconfiguration in progress global indicator
        randomness::initialize(&framework); // randomness seed holder

        let config = randomness_config::new_off();
        randomness_config::initialize(&framework, config);
    }
}
