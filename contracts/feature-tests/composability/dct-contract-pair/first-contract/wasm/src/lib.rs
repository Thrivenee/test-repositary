// Code generated by the dharitri-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    first_contract
    (
        transferToSecondContractFull
        transferToSecondContractHalf
        transferToSecondContractRejected
        transferToSecondContractRejectedWithTransferAndExecute
        transferToSecondContractFullWithTransferAndExecute
        getdctTokenName
        getSecondContractAddress
    )
}

dharitri_wasm_node::wasm_empty_callback! {}
