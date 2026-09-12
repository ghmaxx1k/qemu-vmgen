# qemu-vmgen
a simple application written in Rust to help you set up a qemu VM if you don't know how to

Copyright (C) 2026 ghmaxx1k, distribuited on the GNU/GPLv2 license

# How to install:
    sudo make install

# About
current version: 0.1.0 BETA

current stable version: none

AI code: src/pager.rs, from line 25 to line 37 (used AI snippet as reference)

for future releases: develop an additional virtual machine launcher for created machines
                     add man page

(usage is explained while running qemu-vmgen)

# Changelog

(NFY means not finished yet)

0.0.1 ALPHA: started development,
             added licensing and initial prompt,
             added placeholders for all options,
             titled all code steps/functions.

NFY - 0.1.0 BETA: renamed qemu-helper to qemu-vmgen,
                  license is now compliant,
                  started developing cd and cm features,
                  added a readme,
                  added a makefile,
                  heavily changed code structure (pager and opts are now mods).