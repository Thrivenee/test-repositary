// Code generated by the dharitri-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    crowdfunding_dct
    (
        fund
        status
        getCurrentFunds
        claim
        getTarget
        getDeadline
        getDeposit
        getCrowdfundingTokenIdentifier
    )
}

dharitri_wasm_node::wasm_empty_callback! {}
