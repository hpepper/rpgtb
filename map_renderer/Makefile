TARGET_DIRECTORY := $(shell cargo metadata --format-version 1 | jq | grep target_directory | awk -F\" '{ print $$4 }')

release: ${HOME}/bin/map_renderer

${HOME}/bin/map_renderer: src/main.rs src/map_to_svg.rs
	cargo build --release
	cp ${TARGET_DIRECTORY}/release/map_renderer $@
