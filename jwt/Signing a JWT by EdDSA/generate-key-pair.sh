#!/bin/bash

# Generate a new private key

# for Ed25519
for i in ED25519 ED448; 
do
    openssl genpkey -algorithm $i -out $i-private.pem

    openssl pkey -in $i-private.pem -pubout -out $i-public.pem
done
