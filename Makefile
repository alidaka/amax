.PHONY: push run

push:
	git push && git push github

run:
	cargo run
