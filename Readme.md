<!-- keep this as a h2 title (##), personal aesthetic decision from lead -->
## Mamoru Core

### Init submodules

```shell
make init
```

### Install development dependencies

```shell
## install protoc (compile protobufs)
brew install protobuf

## install buf (update external protobufs)
go install github.com/bufbuild/buf/cmd/buf@v1.9.0

## install node (run AssemblyScript tests)
brew install node
```

### Update Submodules
Mamoru core is composed of multiple submodules. To update them, run the following command (you need to be login using Github SSH in your mamoru account):

```shell
make submodule-update
```

You then will see the changes appearing  in the `mamoru-sniffer/proto` folder and in your version control system.

### Test

There are two test suites: one for general tests (unit tests) and the other one for e2e tests using validation chain

#### run unit tests

```shell
make test
```

#### run e2e tests

to run e2e tests, you need to have a running instance of the validation chain. you can do it by running the following command:

```sh
docker buildx build -t validation-chain -f ./mamoru-sniffer/proto/validation-chain/docker/dev.Dockerfile ./mamoru-sniffer/proto/validation-chain

docker run -p 4500:4500 -p 9090:9090 -p 26657:26657 -p 1317:1317  validation-chain
```

```shell
make validation-chain-test
```

### Format

```shell
cargo fmt --all

## this will check common errors and notify you about it
cargo clippy --workspace --all-features --tests

```
