
run:
	cargo build --release
	cp target/release/sapo ./sapo_compiler

build: 
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
	@ make run
