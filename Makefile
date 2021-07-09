install:
	 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
	 export PATH="$HOME/.cargo/bin"

build:
	cargo build --release

run:
	./target/release/sapo $(ARGS)
