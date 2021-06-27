#!/bin/bash

HOST="localhost:3030"
AUTH="/auth"
NETWORK="/network"
UTIL="/util"
WALLET="/wallet"
HCTJSON="Content-type: application/json"
HACKJSON="Accept: application/json"
APIKEY=aed5d93dd4f75ffc95ccfb1968df9233d682b8a240de5dc7d5ee8308a0b0cd7a

function registration {
    local url="$HOST$AUTH/basic"
    local email="vm@stackmate.in"
    local username=vmd
    local password=secretsauces
    local HAPIKEY="x-sats-api-key:$APIKEY"
    local pass256=$(echo -n $password | sha256sum | rev | cut -c4- | rev)
    local payload="{ \"email\":\"$email\", \"username\":\"$username\", \"pass256\":\"$pass256\" }"
    # echo $payload
    curl -vv -H "$HCTJSON" -H "$HACKJSON" -H "$HAPIKEY" -d "$payload" -X PUT "$url"
}

function config {
    local url="$HOST$AUTH/config"
    local public_key="-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAqkVu2BX3K2ZB+0F+dGor\necTfBM9GYqNxxn3tTMR61fEMBX0vPA5itSQcfh91pofKrvC65CZBnu71EElvo4hU\n9WTqjiaNJJDB3dxLbek2WEx57kCM7vewiwyosUdeBeqdxZX/Tp1xHEyB636J/L4R\nGan7XDGfWs47ZnnmR/CB13LuaHW08ej9WWNiy8UPs0LRzUZkwDNdnhec/N+j5GG0\nTBqwcgfaQDep4irtCuCQ9Q1pXrzgFEwqc0Qsr/F7V2cdJLvtLhG9CW6RZZdlNYbc\nIVNi+G7kVlSts7/81/EsjSAL8VMcvvj6CakBFzyUH4kgQRvlwVA3grL/7d39Wu5F\nBFPVm/40nSMnh28J0Sk/2E5Xt7xSQ9A43WM9mUNLSXkuEZbvMY09yzxzUZo9paPG\nbvKJY72tdmNvc2La0gaEhGlQf+7IDOs9uUBkOw0f+wyzM9bLNiQqLpeQ7cQH9rIT\nV4I+tbo4jEmI5vZwB2AImbsVXEn8z9OxV4TBqBciwi0jECcu5yh6b2cS/Gj7D+5x\nEGvtKO26/Iqpfrzf1Of7unF8DdYz8hZdGZ3Vs3di0apksmwbw7soNk6Q2R/c+c0X\nXneQKZxmDkvOPna1Zldx9n0WSloq+neDdwt0D9DyPORSad1/o1+grg6ksTylX72b\njO+9ZXTV/bfznGJI2ZojOGsCAwEAAQ==\n-----END PUBLIC KEY-----"
    local username=vmd
    local password=secret
    local pass256=$(echo -n $password | sha256sum | rev | cut -c4- | rev)
    local basic_auth=$(echo -n "$username:$pass256" | base64 -w 0)
    # printf "\n$basic_auth\n"
    local HAUTH="Authorization: Basic $basic_auth"
    local HAPIKEY="X-Sats-Api-Key: $APIKEY"
    
    local payload="{ \"public_key\":\"$public_key\" }"
#    echo $payload
    curl -H "$HCTJSON" -H "$HACKJSON" -H "$HAPIKEY" -H "$HAUTH" -d "$payload" -X PUT "$url"


}

function token {
    local url="$HOST$AUTH/token"
    local key_path="$HOME/test_bug.pem"
    local username=vmd
    local password=secret
    local pass256=$(echo -n $password | sha256sum | rev | cut -c4- | rev)
    local time=$(date +%s)
    local message="timestamp=$time"
    local basic_auth=$(echo -n "$username:$pass256" | base64 -w 0)
    local signature=$(echo -ne $message | openssl dgst -sha256 -sign $key_path | openssl base64 -A)
  
    local HAUTH="Authorization: Basic $basic_auth"
    local HAPIKEY="X-Sats-Api-Key: $APIKEY"
    local HTS="X-Sats-Timestamp: $time"
    local HSIG="X-Sats-Client-Signature: $signature"

    # echo $message
 
    curl -H "$HCTJSON" -H "$HACKJSON" -H "$HAPIKEY" -H "$HAUTH" -H "$HSIG" -H "$HTS" -X GET "$url"


}

registration
#printf "\n"
#config
#printf "\n"
#token
#printf "\n"
