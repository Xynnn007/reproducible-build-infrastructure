# SBoM Server

SBoM Server (Software Bill of Materials) server, provides a service to check
a software's reference value, and generate its hash value if check passed.

## structure

`config` Configuration file, indicating the server's address, port and its KV-store type

`proto` protobuf protocols

`src/cache` KV-store traits and instances, by default provide a 'simple'

`src/rvps_handlers` RVPS handlers, including a naive 'in-toto'

## Usage

### Directly Run

Must have a rust compiler, in-toto and docker environment.

#### in-toto environment set-up

```bash
# Fetch the demo repo using git
git clone https://github.com/alibaba/inclavare-containers.git

# Change into the demo directory
cd inclavare-containers/rbi/in-toto/kernel/software-supply-chain-demo

# Install a compatible version of in-toto
pip install -r requirements.txt

# Every step has a default timeout 10s, which is too short for a build
# Change it to 1200s
export IN_TOTO_LINK_CMD_EXEC_TIMEOUT='1200'
```

#### Run

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

### Docker

Build a service image using docker

```bash
docker build -t sbom . --network host
```

Then will build a image with SBoM service image

Run it as a service

```bash
sudo docker run -v /var/run/docker.sock:/var/run/docker.sock \
           -p 7654:7654 \
           -d sbom
```

Then, you can connect `http://127.0.0.1:7654` by grpc to get service.

## Test example

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