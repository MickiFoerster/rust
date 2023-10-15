#!/bin/bash

# Generate a new private key

# for ES256
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out ES256-private.pem

# for ES384
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-384 -out ES384-private.pem

# for ES512
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-521 -out ES512-private.pem

# for ES256K
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:secp256k1 -out ES256K-private.pem

# Generate a public key from the private key.
for i in ES*private.pem; do 
    openssl pkey -in $i -pubout -out $(basename $i -private.pem)-public.pem
done
