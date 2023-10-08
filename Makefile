# this is our root dir, this makefile must stay at the root of the repo
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

APP_NAME:="brunoise"

.PHONY:
	init dev test

init:
	pyoxidizer generate-python-embedding-artifacts pyembedded

dev:
	PYO3_CONFIG_FILE=${ROOT_DIR}/pyembedded/pyo3-build-config-file.txt cargo run

test:
	PYO3_CONFIG_FILE=${ROOT_DIR}/pyembedded/pyo3-build-config-file.txt cargo test
