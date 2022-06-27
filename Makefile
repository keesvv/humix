all: build

build:
	cargo build

dev-archive:
	git archive HEAD | xz > humix-dev-$$(git rev-parse --short HEAD).tar.xz

doc:
	pandoc README.md -o Humix-$$(git rev-parse --short HEAD).pdf