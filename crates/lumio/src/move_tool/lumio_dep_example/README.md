This is a small example of using the new `lumio` dependency. This shall be removed once we have
documentation/tests.

`pack2` contains a package which is used by `pack1` as follows:

```
[dependencies]
Pack2 = { lumio = "http://localhost:8080", address = "default" }
```

To see it working:

```shell
# Start a node with an account
lumio node run-local-testnet &
lumio account create --account default --use-faucet 
# Compile and publish pack2
cd pack2
lumio move compile --named-addresses project=default     
lumio move publish --named-addresses project=default
# Compile pack1 agains the published pack2
cd ../pack1
lumio move compile --named-addresses project=default     
```
