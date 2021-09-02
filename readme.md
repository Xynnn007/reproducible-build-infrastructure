# SBoM Server

SBoM Server (Software Bill of Materials) server, provides a service to check
a software's reference value, and generate its hash value if check passed.

## structure

`config` Configuration file, indicating the server's address, port and its KV-store type

`proto` protobuf protocols

`src/cache` KV-store traits and instances, by default provide a 'simple'

`src/rvps_handlers` RVPS handlers, including a naive 'in-toto'

## Usage

```bash
# run the server
cargo run
```

In another terminal, run

```bash
# run the client
cd tests/bios
cargo run
```

And you will see the output in server
```plaintext
RBI info: v0.1.0
commit: 87c887a05586236c7d6355616d1aa217697dcea6
buildtime: 2021-09-02 16:23:34
Listen gRPC server addr: [::1]:7654
[GRPC] Got a new add-record request. id: 123, class: in-toto
[SimpleKV] search key 123
[in-toto] Got a new tar file to verify...
/tmp/.tmpFM9U6Y
[in-toto] Verification succeeded!
[SimpleKV] Insert 123 -> 48772e82a2993f44894820637ce13e0aceb9ab68d3b01dab79c945eaaa2d74cf
[GRPC] Record added successfully. id: 123, sha256: 48772e82a2993f44894820637ce13e0aceb9ab68d3b01dab79c945eaaa2d74cf
[GRPC] Got a new query request. id: 123
[SimpleKV] search key 123
[GRPC] Query successeed. id: 123, sha256: 48772e82a2993f44894820637ce13e0aceb9ab68d3b01dab79c945eaaa2d74cf
```

The client side
```plaintext
Send record add request...
RESPONSE="48772e82a2993f44894820637ce13e0aceb9ab68d3b01dab79c945eaaa2d74cf"
Send query request...
RESPONSE="48772e82a2993f44894820637ce13e0aceb9ab68d3b01dab79c945eaaa2d74cf"
```