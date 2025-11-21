#Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

clean:
	- rm -rf go.mod go.sum lumiostdlib lumiotypes

test:
	cargo run -p lumio-framework release
	cargo run -p lumio-sdk-builder -- \
		--language go --module-name lumiostdlib \
		--package-name testing \
		--with-lumio-types "../../../../testsuite/generate-format/tests/staged/aptos.yaml" \
		--target-source-dir . \
		"../../../framework/lumio-framework/build/LumioFramework"
	- go mod init testing
	go mod edit -replace testing=`pwd`
	go get github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/bcs
	go get github.com/aptos-labs/serde-reflection/serde-generate/runtime/golang/serde
	go run stdlib_demo.go

.PHONY: clean test
