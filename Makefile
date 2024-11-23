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
	substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events './out' 100000:100500 --encoder parquet --parquet-default-column-compression snappy --file-block-count 100

.PHONY: s3
s3:
	substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events 's3://pinax/eos/48034848cd64dcd70f95e06de9ed5d1478d0133e?region=us-east-1' 2: --encoder parquet --parquet-default-column-compression snappy

.PHONY: schema
schema:
	substreams-sink-files tools parquet schema substreams.yaml map_events