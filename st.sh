#!/bin/bash
tokei -f -slines -c90 -tPython,Rust -etarget
grep --color=auto --exclude-dir=target --include=*.rs -r format......,
git st
