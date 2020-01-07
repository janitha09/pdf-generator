nocapture:
	cargo test -- --nocapture

rocketcodegen:
	export ROCKET_CODEGEN_DEBUG=1 cargo build

# https://rocket.rs/v0.3/guide/configuration/
# [dependencies]
# rocket = { version = "0.3.17", features = ["tls"] }

rockettls:
	export ROCKET_TLS={certs="/path/to/certs.pem",key="/path/to/key.pem"} cargo run