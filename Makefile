PREFIX?=/usr/local
OPTS?=

INSTALL_BIN_DIR=$(PREFIX)/bin
INSTALL=install

CARGO=$(or $(shell which cargo 2> /dev/null),/usr/local/bin/cargo)

PROJ_ETC_DIR=./etc

SYSFS_SERVICE=jarvis-sysfs-permissions.service
SYSD_SERVICE_DIR=/usr/lib/systemd/system

TARGET_BIN=./target/release/jarvis

all: build

build:
	$(CARGO) build --release $(OPTS)

clean:
	$(CARGO) clean $(OPTS)

install: all
	sudo $(INSTALL) -m 0644 $(PROJ_ETC_DIR)/$(SYSFS_SERVICE) -t $(SYSD_SERVICE_DIR)
	sudo systemctl enable --now $(SYSFS_SERVICE)
	sudo $(INSTALL) $(TARGET_BIN) -t $(INSTALL_BIN_DIR)

uninstall:
	sudo systemctl disable --now $(SYSFS_SERVICE)
	sudo rm $(SYSD_SERVICE_DIR)/$(SYSFS_SERVICE)
	sudo rm $(INSTALL_BIN_DIR)/jarvis
