# julie

*NOTE*: This project is in extereme BETA. Currently only for learning. It will be versioned such that v^1 will be stable for production use.

## overview

a simple multi-factor authentication server, inspired by Kerberos.

## test

### units
```bash
cargo test -- --test-threads 1
# this is required to ensure that the service tests run after sled so that a db is created
```

### integration
To test the server with the bash client, first create a new user with an apikey

```bash
# Uncomment the last two asserts that delete the clinet and service db entries
nano auth/core.rs
# Run the unit test
cargo test -- --nocapture core_composite
# Copy the apikey value into the APIKEY varibale
nano test/auth.bash
# Comment out delete again so successive tests will clean up
nano auth/core.rs
# Start the server
cargo run
# Run the server test
bash test/auth.bash
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

    - `handler`: Provides http wrappers for core.

    - `router`: Declares the http api exposed by the `auth` module.


### priorities

1. correct-tooling: 

Our primary focus is on creating correct `lib` tools for all the provided authentication methods. This is the core of the project and also the easiest way to contribute as they are independently testable units. All lib units must return  `Result` types and properly handle errors and not panic.

2. async-server: 

We want to create a correct implementation of our chosen http server library (warp) and create an async friendly http service. `async` in rust is still new to us and we are certainly not using it as effectively as we could. 

3. database-interface: finally, it would be nice to decouple `sled` from the storage model.

