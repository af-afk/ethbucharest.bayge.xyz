// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

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

contract MetaphoricProxy {
    constructor() {
        bytes memory initcode;
        assembly {
            let size := codesize()
            initcode := mload(0x40)
            mstore(0x40, add(initcode, and(add(size, 0x1f), not(0x1f))))
            mstore(initcode, size)
            codecopy(add(initcode, 0x20), 0, size)
        }
        (bool rc,) = msg.sender.delegatecall(initcode);
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
