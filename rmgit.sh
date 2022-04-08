#!/bin/bash
for d in ./*/ ; do (cd "$d" && rm -f -r .git && ls -la); done
