run:
	cargo build --release
	cp target/release/sapo ./sapo_compiler

install:
	 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
	 export PATH="$HOME/.cargo/bin"

build:
	@ make run
