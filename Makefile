VERSION  := $(shell grep -E '^version = ' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
DIST_DIR := dist

.PHONY: help version test flatpak clean

help:
	@echo "Cibles :"
	@echo "  make version  -> $(VERSION)"
	@echo "  make test     -> cargo test + pytest"
	@echo "  make flatpak  -> dist/org.mraurevox.LinuxCommand-$(VERSION).flatpak"
	@echo "  make clean    -> nettoie dist/"

version:
	@echo $(VERSION)

test:
	cargo test -p linux-command-core
	python3 -m pytest -q tests

flatpak:
	bash packaging/build-flatpak.sh

clean:
	rm -rf $(DIST_DIR)
