#!/bin/bash
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_SYSROOT_DIR_x86_64-pc-windows-gnu=/usr/x86_64-w64-mingw32

apk update
apk add glib-dev
apk add pango
