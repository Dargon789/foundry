// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.18;

import "ds-test/test.sol";
import "cheats/Vm.sol";

// https://github.com/foundry-rs/foundry/issues/3708
contract Issue3708Test is DSTest {
    // https://optimistic.etherscan.io/address/0x4e59b44847b379578588920ca78fbf26c0b4956c#code
    address constant CREATE2_DEPLOYER = 0x4e59b44847b379578588920cA78FbF26c0B4956C;
    Vm constant vm = Vm(HEVM_ADDRESS);

    function setUp() public {
        uint256 forkId = vm.createSelectFork("optimism");

        bytes memory code =
            hex"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3";
        assertEq(CREATE2_DEPLOYER.code, code);
    }

    function test_deployer() public {
        bytes memory code =
            hex"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3";
        assertEq(CREATE2_DEPLOYER.code, code);
    }
}
