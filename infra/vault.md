# Setup hashicorp vault 

Create a policy for the client (authentication) service

We follow the paths used in the docker setup: 

```bash
nano /vault/config/policies/julie/test/client.hcl
```

```hcl
path "julie/test/client/*" {
  capabilities = ["create","read", "update", "delete","list"]
}

```

Create a policy for the service (ticket) service


```bash
nano /vault/config/policies/julie/test/service.hcl
```

```hcl
path "julie/test/service/*" {
  capabilities = ["create","read", "update", "delete","list"]
}

```

Then create these secret engines:

```bash

vault secrets enable -path=julie-test-client kv
# Success! Enabled the kv secrets engine at: julie-test-client
vault secrets enable -path=julie-test-service kv
# Success! Enabled the kv secrets engine at: julie-test-service
```

Write the policies we created:


```bash
vault policy write julie-test-client /vault/config/policies/julie/test/client.hcl
# Success! Uploaded policy: julie-test-client
vault policy write julie-test-service /vault/config/policies/julie/test/service.hcl
# Success! Uploaded policy: julie-test-service

```

Issue tokens for each policy

```bash
vault token create -policy=julie-test-client
# Key                  Value
# ---                  -----
# token                s.IvslwhG65dfQcRigKZ8iBPT6
# token_accessor       u8CPidOfF6kTsyT1IjUWZkFj
# token_duration       768h
# token_renewable      true
# token_policies       ["default" "julie-test-client"]
# identity_policies    []
# policies             ["default" "julie-test-client"]


vault token create -policy=julie-test-service
#  Key                  Value
#  ---                  -----
# token                s.focBoVGrW0iUT7HxJa0qVdIm
# token_accessor       8V0QBGFqRUAGqjvKqOLAotHH
# token_duration       768h
# token_renewable      true
# token_policies       ["default" "julie-test-service"]
# identity_policies    []
# policies             ["default" "julie-test-service"]


```

ThaTs IT! Julie can now use these tokens and paths to initialize a VaultStorage which impl JulieStorage.


