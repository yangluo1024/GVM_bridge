# GVM-Bridge
Integration through VM inter-modulation protocols

### Overview
* There are a number of projects on Ethereum that are developed in solidaty and run on EVM. To take advantage of these old resources, an EVM implementation has been integrated in the Polkadot project system. However, the problem now is that WASM contracts and EVM contracts cannot call each other, which limits the integration between WASM contract resources and EVM contract resources.
* With the development of technology, Polkadot is likely to support more new smart contract VMs in the future, and there is also the problem of resource integration with the original VM contracts.
* Therefore, there is a need to develop a system (Generic VM Bridge - GVM Bridge) and a standard (VM General ABI - VM GABI) to achieve resource integration through VM inter-modulation protocols, so that the resources of various different VMs can be used with each other immediately without migration, thus improving the utilization of resources.

### Project Details
Through the GVM Bridge system and the VM interconnection protocol, smart contracts between VMs can be invoked to achieve resource consolidation, and resources from different VMs can be used immediately without migration, thus improving resource utilization.

#### **Technical Design Overview**
GVM Bridge will provide the GVM Bridge Framework component and the ABI conversion tools Trans4WABI and Trans4EABI for converting WASM ABI to VM GABI and EVM ABI to VM GABI, respectively.
The WASM-Proxy pallet and the EVM-Proxy pallet specifically implement the WASM VM and EVM calls.

The functions of these components are:
#### **GVM Bridge Framework**
1. Build inter-call framework for different VM contract to provide the interface for virtual machine proxy access.
2. Implement VM proxy registration.
3. Provide encapsulation of public standard components AccountId, Block, etc.
4. Provide ABI conversion access interface.
5. Provide access to contract invocation interface.

#### **WASM-Proxy pallet**
1. Implement the GVM Bridge Framework interface.
2. Implement the inter-conversion of VM GABI parameters and WASM ABI parameters.
3. Implement the inter-conversion between public standard components and WASM components.
4. Implement the host function for contract inter-call.

#### **EVM-Proxy pallet**
1. Implement the GVM Bridge Framework interface.
2. Implement the inter-conversion of VM GABI parameters and EVM ABI parameters.
3. Implement the inter-conversion between public standard components and EVM components.
4. Implement the pre-compiled contract for contract inter-calls.

#### **Trans4WABI**
1. Implement WASM ABI to VM GABI conversion.
2. Implement VM GABI to WASM ABI conversion.

#### **Trans4EABI**
1. Implement EVM ABI to VM GABI conversion.
2. Implement VM GABI to EVM ABI conversion.

-------
#### **License**
The project are currently licensed under [BSL](https://github.com/CycanTech/GVM-Bridge/blob/main/License)<br>
Test cases(`tests.*`) are licensed under [Apache 2.0](https://github.com/CycanTech/GVM-Bridge/blob/main/License-APACHE2)

#### **Other**
In order to facilitate developers to develop contract inter-call function, we will provide sample codes for WASM contract and EVM contract inter-calling, such as inter-calling ERC20 contract etc.
