# See https://github.com/cosmos/cosmos-sdk/blob/main/proto/README.md
COSMOS_SDK_COMMIT="8cb30a2c4de74dc9bd8d260b1e75e176"

init:
	git submodule init

pull-proto-dependencies:
	buf export buf.build/cosmos/cosmos-sdk:$(COSMOS_SDK_COMMIT) --output ./mamoru-core/proto/
