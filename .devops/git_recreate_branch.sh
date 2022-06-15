#!/usr/bin/env bash

cd -P "$(dirname "${0-$BASHSOURCE}")/.." || exit 1

CURRENT_BRANCH="$(git branch --show-current)"
[[ "${CURRENT_BRANCH}" == 'master' ]] \
  && echo "NEVER run this on master branch." \
  && exit 1;

git checkout --orphan latest_branch
git add -A
git commit -am "Recreate branch without version history."
git branch -D "${CURRENT_BRANCH}"
git branch -m "${CURRENT_BRANCH}"
while read -r remote ; do
  git push -f "${remote}" "${CURRENT_BRANCH}"
done< <(git remote)