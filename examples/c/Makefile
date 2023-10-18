
main: main.c generate_header
	mkdir -p out/
	cc $< -o $@ --output=out/c_example -L ../../target/release -l c_space_physics_engine -l pthread -l dl -lm

.PHONY: build_lib
build_lib:
	cargo build -p c_space_physics_engine --release

generate_header: build_lib
	#cargo test -p c_space_physics_engine --features headers -q -- generate_headers
	cargo run -p c_space_physics_engine --features headers --bin generate-headers

clean:
	cargo clean
	$(RM) main

run: main
	./out/c_example