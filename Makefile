ENV=
MY_FILE := rust_web

all: deploy

run:
	${ENV} cargo watch -x run

deploy: fclean build
	@if [ ! -e $(MY_FILE) ]; then \
		echo "Le fichier $(MY_FILE) n'existe pas, arrêt du processus Make."; \
		exit 1; \
	fi
	docker-compose up --build -d

build:
	cargo build --release
	cp ./target/release/rust_web .

clean:
	rm -rf ./target/

fclean: clean 
	rm -rf ./rust_web
