run:
	docker run -it --rm -v $(PWD):/app -w /app rust cargo run --bin $(bin)

new:
	@{ \
	read -p "What day of December is it? " day; \
	cp src/template.rs src/day$$day.rs; \
	echo "\n[[bin]]" >> Cargo.toml; \
	echo "name = \"day$$day\"" >> Cargo.toml; \
	echo "path = \"src/day$$day.rs\"" >> Cargo.toml; \
	echo "Added binary target day$$day to Cargo.toml"; \
	}

interact:
	docker run -it --rm -v $(PWD):/app -w /app rust bash
