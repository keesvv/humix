all: build

build:
	cargo build

dev-archive:
	git archive HEAD | xz > humix-dev-$$(git rev-parse --short HEAD).tar.xz