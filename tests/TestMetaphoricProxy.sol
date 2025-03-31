// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "forge-std/console.sol";

import "../src/MetaphoricProxy.sol";

contract StorageProxy {
    address admin;
}

contract ContractMetaphoricalProxy is StorageProxy {
    function swag() external view returns (address) {
         return admin;
    }
}

contract FactoryMetaphoricProxy is StorageProxy, Test {
    function deploy() external returns (ContractMetaphoricalProxy) {
        return ContractMetaphoricalProxy(address(
            new MetaphoricProxy(0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7)
        ));
    }

    function setup(address _admin) external {
        admin = _admin;
    }
}

contract TestMetaphoricalProxy is Test {
    function testDeploy() public {
        ContractMetaphoricalProxy impl = new ContractMetaphoricalProxy();
        vm.etch(0x9999999999999999999999999999999999999999, address(impl).code);
        impl.swag();
        FactoryMetaphoricProxy p = new FactoryMetaphoricProxy();
        ContractMetaphoricalProxy c = p.deploy();
        vm.assertEq(0x6221A9c005F6e47EB398fD867784CacfDcFFF4E7, c.swag());
    }
}
