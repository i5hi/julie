# julie-server

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
# Uncomment delete so the new user from init_bash_test persists
nano services/auth.rs
# Run the unit test
cargo test -- --nocapture init_bash_test
# Copy the apikey value into the APIKEY varibale
nano test/auth.bash
# Comment out delete again so successive tests will clean up
nano services/auth.rs
# Start the server
cargo run
# Run the server test
bash test/auth.bash
```


### use-cases

If you have a few different services all requiring authentication, offload all that work to kuppy.

Each 'micro-service' within your infrastructure registers a `shared_secret` with the `ticket management service (ticket)` and required `AuthLevel` (using `julie-cli`). 

Clients to your 'micro-services' register with the kuppyd `authentication service (auth)`. On every authentication request, they specify which 'micro-service' they wish to connect with and perform the required authentication. If successful, the `auth` service forwards the request to the `ticket` service which issues an authentication token for the client to authenticate at the requested 'micro-service'.

Each 'micro-service' now only needs to know how to verify a JWT token rather than performing all the various authentication methods natively. 

`julie` diverts from the classical Kerberos model primarily for the sake of simplicity. With `julie-server`, the client only interacts with the `auth` service, which in turn forwards requests to the `ticket` service if authentication is successful. Your 'micro-service' also cannot directly interact with the `ticket` service to register and update their `shared_secret`. It must do so via an admin using `julie-cli`. 

## goals

kuppyd strives to be:

- small:

Low dependencies & efficient resource usage

- correct:

Extensively tested

- flexible:

Easy to extend and upgrade

## structure

- `lib`:

Contains all core modules and tools required by the `auth` and `ticket` services. Can be independently tested. 

- `auth`: 

    - `storage`: Defines the data model and storage for `auth` and `ticket` service.

    - `router`: Declares the http api exposed by the `auth` and `ticket` service.

    - `handlers`: Http wrappers and core logic for each service. Core logic can be decoupled from http implementation by moving it to a `service` file. 

### priorities

1. core: 

Our primary focus is on creating correct `lib` tools for all the provided authentication methods. This is the core of the project and also the easiest way to contribute as they are independently testable units. All lib units must return  `Result` types and properly handle errors and not panic.

2. async-server: 

We want to create a correct implementation of our chosen http server library (warp) and create an async friendly http service. `async` in rust is still new to us and we are certainly not using it as effectively as we could. Currently only the `auth` services exposes an http api. To register 'micro-services' with the `ticket` service, you are required to use `julie-cli`.

3. interfaces: finally, it would be nice to decouple `sled` from the storage model and `warp` from the core-logic by creating an interface for both. This will allow users to implement alternative server and databases of their choice - without having to meddle with core logic.

