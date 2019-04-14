.PHONY: push run install clean

push:
	git push && git push github

run:
	foreman start

install:
	cd frontend && yarn install

clean:
	cd backend && cargo clean && cd ../frontend && yarn clean
