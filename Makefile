.PHONY: all examples basic-usage series-observations update-swagger clean

all: examples

examples: basic-usage series-observations

basic-usage:
	cargo run --example basic_usage

series-observations:
	cargo run --example series_observations

update-swagger:
	npm install @openapitools/openapi-generator-cli -g
	openapi-generator generate \
		-i docs/fred-api-swagger.yaml \
		-g rust \
		--library reqwest \
		-o fred-api-rs

clean:
	cargo clean
