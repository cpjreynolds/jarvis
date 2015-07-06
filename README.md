# Jarvis
Just Another Very Intelligent System

# Installation

```sh
$ make all && make install
```

> **Note:** Jarvis will require sudo priveleges during installation to
configure its systemd service and will `make install` will prompt appropriately.
In addition, you may need to use `sudo make install` if you do not normally have
permission to modify the destination directory. The install location can be adjusted
by passing a PREFIX="..." argument to `make install`.
