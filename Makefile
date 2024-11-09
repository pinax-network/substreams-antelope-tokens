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

.PHONY: gui
gui:
	substreams gui substreams.yaml -e eos.substreams.pinax.network:443 map_events -s -1000 -t 0

.PHONY: parquet
parquet:
	substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events '.' 2: --encoder parquet

.PHONY: schema
schema:
	substreams-sink-files tools parquet schema substreams.yaml map_events