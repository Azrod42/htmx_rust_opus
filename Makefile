ENV=

all: prod

run:
	${ENV} cargo watch -x run

c: check

check:
	cargo check

prod: clean
	cargo build --release
	docker-compose up --build -d

build:
	cargo build --release
	cp ./target/release/rust_web .
	${ENV} ./rust_web

clean:
	rm -rf ./target/

fclean: clean 
	rm ./scop
