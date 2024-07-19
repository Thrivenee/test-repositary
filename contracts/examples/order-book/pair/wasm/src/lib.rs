// Code generated by the dharitri-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

dharitri_wasm_node::wasm_endpoints! {
    order_book_pair
    (
        createBuyOrder
        createSellOrder
        matchOrders
        cancelOrders
        cancelAllOrders
        freeOrders
        startGlobalOperation
        stopGlobalOperation
        getAddressOrderIds
        getOrderIdCounter
        getOrderById
        getFirstTokenId
        getSecondTokenId
    )
}

dharitri_wasm_node::wasm_empty_callback! {}