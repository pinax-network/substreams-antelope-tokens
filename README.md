# Antelope `eosio.token` Substream

> Antelope `eosio.token` tokens

### [Latest Releases](https://github.com/pinax-network/substreams-antelope-tokens/releases)

### Quickstart

```bash
$ make
$ make gui
```

### Mermaid graph

```mermaid
graph TD;
  map_transfers[map: map_transfers];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_transfers;
  map_accounts[map: map_accounts];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_accounts;
  map_stat[map: map_stat];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_stat;
  graph_out[map: graph_out];
  map_accounts --> graph_out;
  map_stat --> graph_out;
  map_transfers --> graph_out;
```

### Modules

```yaml
Package name: antelope_tokens
Version: v0.3.1
Doc: Antelope `eosio.token` based action traces & database operations.
Modules:
----
Name: map_transfers
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.TransferEvents
Hash: 207453ead04995fab056af784c0167d99e635a0d

Name: map_accounts
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Accounts
Hash: e5e8049511bca553f070e6b5e84c2df4d243a87c

Name: map_stat
Initial block: 0
Kind: map
Output Type: proto:antelope.eosio.token.v1.Stats
Hash: a8291577cc0f687f8c07ff9d944c8685dbbbbda9

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 6ccd6bb44d4cba8572bece4c677075d3029fea75
```
