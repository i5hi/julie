#!/bin/bash

HOST="localhost:3030"
AUTH="/auth"

CTJSON="Content-type: application/json"
ACKJSON="Accept: application/json"
ACKURL="Accept: application/x-www-form-urlencoded"

APIKEY=b9483feec625392d1b2270e52e7212f966325b7ed7e938e5938cd1f652c62455

function registration {
    local url="$HOST$AUTH/basic"
    local email="vm@stackmate.in"
    local username=vmd
    local password=secret
    local HAPIKEY="x-sats-api-key:$APIKEY"
    local pass256=$(echo -n $password | sha256sum | rev | cut -c4- | rev)
    local payload="{ \"email\":\"$email\", \"username\":\"$username\", \"pass256\":\"$pass256\" }"
    # echo $payload
    curl -H "$CTJSON" -H "$ACKJSON" -H "$HAPIKEY" -d "$payload" -X PUT "$url"
}

function config {
    local url="$HOST$AUTH/pubkey"
    local public_key="-----BEGIN PUBLIC KEY-----\nMIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAqkVu2BX3K2ZB+0F+dGor\necTfBM9GYqNxxn3tTMR61fEMBX0vPA5itSQcfh91pofKrvC65CZBnu71EElvo4hU\n9WTqjiaNJJDB3dxLbek2WEx57kCM7vewiwyosUdeBeqdxZX/Tp1xHEyB636J/L4R\nGan7XDGfWs47ZnnmR/CB13LuaHW08ej9WWNiy8UPs0LRzUZkwDNdnhec/N+j5GG0\nTBqwcgfaQDep4irtCuCQ9Q1pXrzgFEwqc0Qsr/F7V2cdJLvtLhG9CW6RZZdlNYbc\nIVNi+G7kVlSts7/81/EsjSAL8VMcvvj6CakBFzyUH4kgQRvlwVA3grL/7d39Wu5F\nBFPVm/40nSMnh28J0Sk/2E5Xt7xSQ9A43WM9mUNLSXkuEZbvMY09yzxzUZo9paPG\nbvKJY72tdmNvc2La0gaEhGlQf+7IDOs9uUBkOw0f+wyzM9bLNiQqLpeQ7cQH9rIT\nV4I+tbo4jEmI5vZwB2AImbsVXEn8z9OxV4TBqBciwi0jECcu5yh6b2cS/Gj7D+5x\nEGvtKO26/Iqpfrzf1Of7unF8DdYz8hZdGZ3Vs3di0apksmwbw7soNk6Q2R/c+c0X\nXneQKZxmDkvOPna1Zldx9n0WSloq+neDdwt0D9DyPORSad1/o1+grg6ksTylX72b\njO+9ZXTV/bfznGJI2ZojOGsCAwEAAQ==\n-----END PUBLIC KEY-----"
    local username=vmd
    local password=secret
    local pass256=$(echo -n $password | sha256sum | rev | cut -c4- | rev)
    local basic_auth=$(echo -n "$username:$pass256" | base64 -w 0)
    # printf "\n$url\n"
    local HAUTH="authorization: Basic $basic_auth"
    local HAPIKEY="x-sats-api-key: $APIKEY"
    
    local payload="{ \"public_key\":\"$public_key\" }"
#    echo $payload
    curl -H "$CTJSON" -H "$ACKJSON" -H "$HAPIKEY" -H "$HAUTH" -d "$payload" -X PUT "$url"


}

function token {
    local url="$HOST$AUTH/token?service=satoshiplay"
    local key_path="$HOME/test_bug.pem"
    local username=vmd
    local password=secret
    local pass256=$(echo -n $password | sha256sum | rev | cut -c4- | rev)
    local time=$(date +%s)
    local message="timestamp=$time"
    local basic_auth=$(echo -n "$username:$pass256" | base64 -w 0)
    local signature=$(echo -ne $message | openssl dgst -sha256 -sign $key_path | openssl base64 -A)
  
    local HAUTH="Authorization: Basic $basic_auth"
    local HAPIKEY="x-sats-api-key: $APIKEY"
    local HTS="x-sats-timestamp: $time"
    local HSIG="x-sats-client-signature: $signature"

    # echo $message
 
    curl -H "$HAPIKEY" -H "$HAUTH" -H "$HSIG" -H "$HTS" -X GET "$url"


}

decode_base64_url() {
  local len=$((${#1} % 4))
  local result="$1"
  if [ $len -eq 2 ]; then result="$1"'=='
  elif [ $len -eq 3 ]; then result="$1"'=' 
  fi
  echo "$result" | tr '_-' '/+' | openssl enc -d -base64
}

decode_jwt(){
   decode_base64_url $(echo -n $2 | cut -d "." -f $1) | jq .
}


# registration
printf "\n"
# config
printf "\n"
printf "\n"

TOKEN=$(token | jq -r ".token")

printf "$TOKEN"




printf "\n"

