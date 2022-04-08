#!/bin/bash
for d in ./*/ ; do (cd "$d" && rm -f -r .git && rm -f .gitignore && ls -la); done
