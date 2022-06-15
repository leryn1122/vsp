#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")/.." || exit 1

CURRENT_BRANCH="$(git branch --show-current)"

while read -r remote ; do
  git push "${remote}"  "${CURRENT_BRANCH}"
done< <(git remote)