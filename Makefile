run:
	docker run -it --rm -v $(PWD):/app -w /app rust cargo run --bin $(bin)

new:
	@read -p "What day is it?" name; \
	cp src/template.rs src/day$$name.rs

interact:
	docker run -it --rm -v $(PWD):/app -w /app rust bash
