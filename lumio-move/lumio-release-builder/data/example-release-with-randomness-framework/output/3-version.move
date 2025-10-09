// Script hash: c2035ec4
script {
    use lumio_framework::lumio_governance;
    use lumio_framework::version;

    fun main(core_resources: &signer) {
        let core_signer = lumio_governance::get_signer_testnet_only(core_resources, @0x1);

        let framework_signer = &core_signer;

        version::set_for_next_epoch(framework_signer, 999);
        lumio_governance::reconfigure(framework_signer);
    }
}
