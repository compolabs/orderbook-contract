contract;

use sway_libs::upgradability::{
    _proxy_owner,
    _proxy_target,
    _set_proxy_owner,
    _set_proxy_target,
    only_proxy_owner,
};
use standards::{src14::{SRC14, SRC14Extension}, src5::State};
use std::execution::run_external;

storage {
    SRC14 {
        /// The [ContractId] of the target contract.
        ///
        /// # Additional Information
        ///
        /// `target` is stored at sha256("storage_SRC14_0")
        target in 0x7bb458adc1d118713319a5baa00a2d049dd64d2916477d2688d76970c898cd55: Option<ContractId> = None,
        /// The [State] of the proxy owner.
        ///
        /// # Additional Information
        ///
        /// `proxy_owner` is stored at sha256("storage_SRC14_1")
        proxy_owner in 0xbb79927b15d9259ea316f2ecb2297d6cc8851888a98278c0a2e03e1a091ea754: State = State::Uninitialized,
    },
}

abi ProxyOwner {
    #[storage(read, write)]
    fn set_proxy_owner(new_proxy_owner: State);
}

impl SRC14 for Contract {
    #[storage(read, write)]
    fn set_proxy_target(new_target: ContractId) {
        only_proxy_owner();
        _set_proxy_target(new_target);
    }

    #[storage(read)]
    fn proxy_target() -> Option<ContractId> {
        _proxy_target()
    }
}

impl ProxyOwner for Contract {
    #[storage(read, write)]
    fn set_proxy_owner(new_proxy_owner: State) {
        // checking twice because don't control sway_libs
        only_proxy_owner();
        _set_proxy_owner(new_proxy_owner);
    }
}

impl SRC14Extension for Contract {
    #[storage(read)]
    fn proxy_owner() -> State {
        _proxy_owner()
    }
}

#[fallback]
#[storage(read)]
fn fallback() {
    // pass through any other method call to the target
    run_external(_proxy_target().unwrap())
}
