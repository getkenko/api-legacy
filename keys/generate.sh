#!/bin/bash

PRIV_NAME="private.pem"
PUB_NAME="public.pem"

echo "==> Generating private key..."
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:prime256v1 -out $PRIV_NAME

echo "==> Extracting public key..."
openssl pkey -in $PRIV_NAME -pubout -out $PUB_NAME

echo "==> Key pair generated!"