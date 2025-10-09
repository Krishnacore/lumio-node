// Initialize AIP-67 parital governance voting.
script {
    use lumio_framework::lumio_governance;
    use lumio_framework::jwks;

    fun main(proposal_id: u64) {
        let framework_signer = lumio_governance::resolve_multi_step_proposal(
            proposal_id,
            @0x1,
            {{ script_hash }},
        );
        jwks::initialize(&framework_signer);
    }
}
