TARGET_DIRECTORY := $(shell cargo metadata --format-version 1 | jq | grep target_directory | awk -F\" '{ print $$4 }')

release: ${HOME}/bin/security_sheaf_generator

${HOME}/bin/security_sheaf_generator: src/main.rs
	cargo build --release
	cp ${TARGET_DIRECTORY}/release/security_sheaf_generator $@

test: ${HOME}/bin/security_sheaf_generator
	${HOME}/bin/security_sheaf_generator --security_code blue --security_value 7