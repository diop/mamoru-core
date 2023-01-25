## Mamoru Core

### Init submodules

```shell
make init
```

### Install development dependencies

```shell
# install protoc (compile protobufs)
brew install protobuf

# install buf (update external protobufs)
go install github.com/bufbuild/buf/cmd/buf@v1.9.0

# install node (run AssemblyScript tests)
brew install node
```
