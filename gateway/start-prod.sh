#!/bin/bash

function cleanup {
  kill "$ACCOUNTS_PID"
  kill "$PRODUCTS_PID"
  kill "$REVIEWS_PID"
}

trap cleanup EXIT

cd ../federation-customer
cargo build --bin federation-customer
CUSTOMER_SERVICE_ADDRESS=http://customer.service.efishery.com cargo run --bin federation-customer &
ACCOUNTS_PID=$!

cd ../federation-cultivation
cargo build --bin federation-cultivation
CULTIVATION_SERVICE_ADDRESS=http://cultivation.service.efishery.com cargo run --bin federation-cultivation &
PRODUCTS_PID=$!

cd ../gateway
sleep 3

node index.js
