.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eos.substreams.pinax.network:443 map_transfers -s -1

.PHONY: gui
gui:
	substreams gui -e eos.substreams.pinax.network:443 map_transfers -s -1

.PHONY: accounts
accounts:
	substreams gui -e eos.substreams.pinax.network:443 map_accounts -s -1

.PHONY: stat
stat:
	substreams gui -e eos.substreams.pinax.network:443 map_stat -s -10000

.PHONY: graph_out
graph_out:
	substreams gui -e eos.substreams.pinax.network:443 graph_out -s -1
