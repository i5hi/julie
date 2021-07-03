# julie

*NOTE*: This project is in extereme BETA. Currently only for learning. It will be versioned such that v^1 will be stable for production use.

## overview

a simple multi-factor authentication server, inspired by Kerberos.

## test

### units

### ENSURE THE FOLLOWING TESTS ARE IGNORED IF YOU ARE RUNNING TESTS ON PROD:

- src/auth/client/delete_all_clients
- src/auth/service/delete_all_services
- src/auth/core/core_email_composite
- src/lib/email/test_sendmail

*They should be ignored by default, unless you have changed it.*

To run all invidual unit tests:

```bash
cargo test --bin jd -- --test-threads 1
# test-threads: required to ensure that the service tests run after sled so that a db is created.
# bin: required to ensure the suite runs only once. Since we have two bins, it will run twice by default.

# ALTERNATIVELY
bash test/all.bash
# will do the above

```


### integration
To test the daemon with the bash client, first create a new client with the cli

```bash
# Uncomment the last two asserts that delete the clinet and service db entries
cargo run --bin jc client register
cargo run --bin jc service register satoshiplay 122ded04a4818942ca52f8844e86df65fe5db3bb4b66bb45a4b02aea6e1bdef5

# Add the api key to the test client file
nano test/auth_signer.bash
# Start the daemon
cargo run --bin jd
# Run the integration test
bash auth_signer.bash
# NOTE: THIS TEST MUTATES YOUR STATE. 
# Investigate using the cli tool and clean up with the delete subcommands
```

## build

```bash
bash build.sh

# start the server
jd
# use the client
jc info
jc util random
jc client list
jc server register cyphernode 5608f6ad1e6b71514ee3c465061f4a471f59a76734796e5d78c7191cedd30127
jc server list


```
## goals

julie strives to be:

- small:

Low dependencies & efficient resource usage

- correct:

Extensively tested

- flexible:

Easy to extend and upgrade

## structure

- `lib`:

Contains all core modules and tools required by the `auth` module. Can be independently tested. 

- `auth`: 

    - `client`: Defines the data model and storage for a client.

    - `service`: Defines the data model and storage for a service.

    - `core`: Defines all the core logic for the auth module

    - `dto`: Provides http wrappers for core. Abbreviates to Data-Transfer-Object. Converting IO from Http to Native.

    - `router`: Declares the http api exposed by the `auth` module.

- `storage`: 

    - `interface`: Defines the requirements of storage devices

    - `sled`: SledDB implementation of the storage interface

### priorities

1. correct-tooling: 

Our primary focus is on creating correct `lib` tools for all the provided authentication methods. This is the core of the project and also the easiest way to contribute as they are independently testable units.

2. async-server: 

We want to create a correct implementation of our chosen http server library (warp). async in rust is still new to us and we are certainly not using it as effectively as we could. Currently the `lib` and `auth/core` of the julie is not async, only `handlers` and `daemon.rs`.  


### known bugs

- Server Rejections: 

Correctly handled errors currently get logged as ERROR in tracing becuase of how we handle warp::Reply and warp::Response in the dto. 

Also, handle_rejection() is now chained to the end of all the routes. 
This error is suppressed if it is chained to each route individually but this brings up another error where if a request for route #3 is made, route#1's handle_rejection completes that request with NotFound. 

For now, this is okay, its just that it confuses, tracing, the actual request gets handled correctly. 

This is warp's main drawback. The filter chain means that requests pass through each route by order so request to route #5 always goes through the first 4 routes. This doesnt scale well. 

