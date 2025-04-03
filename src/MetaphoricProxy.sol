// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/console.sol";

bytes32 constant SLOT_IMPL = bytes32(uint256(keccak256('eip1967.proxy.implementation')) - 1);

library StorageSlot {
    struct AddressSlot {
        address value;
    }
    function getAddressSlot(bytes32 slot) internal pure returns (AddressSlot storage r) {
        assembly {
            r.slot := slot
        }
    }
}

interface MetaphoricProxyCallback {
    function setup(address) external;
}

contract MetaphoricProxy {
    constructor(address _admin) {
        StorageSlot.getAddressSlot(SLOT_IMPL).value = 0x9999999999999999999999999999999999999999;
        (bool rc,) = msg.sender.delegatecall(abi.encodeWithSelector(
            MetaphoricProxyCallback.setup.selector,
            _admin
        ));
        require(rc);
    }

    fallback() external {
        (bool rc, bytes memory rd) =
            StorageSlot.getAddressSlot(SLOT_IMPL).value.delegatecall(msg.data);
        if (rd.length > 0 && !rc) {
            assembly {
                revert(add(rd, 0x20), mload(rd))
            }
        } else {
            require(rc);
            if (rd.length > 0) {
                assembly {
                    return(add(rd, 0x20), mload(rd))
                }
            }
        }
    }
}
