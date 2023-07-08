#! /usr/bin/env bash

NEW_VERSION=$1
if [ -z "${NEW_VERSION}" ]; then
	echo "Usage: ./prepare-release.sh NEW_VERSION_NUMBER"
	exit 1
fi

git switch develop
git fetch origin
git diff origin/develop | if [ "$0" = "" ]; then
    echo "git diff found local changes on develop branch, cannot cut release."
elif [ "$NEW_VERSION" = "" ]; then
    echo "No version set. Are you just copying and pasting this without checking?"
else
    git pull origin develop --ff-only
    git switch -c "prepare-${NEW_VERSION}"
   
    # exit early if the script fails 
    ./update-versions.sh "${NEW_VERSION}" || exit $?

    # sanity check the diffs, but not Cargo.lock files cuz ugh
    git diff -- . ':(exclude)Cargo.lock'

    # send it all to github
    git commit -a -m "Update version to ${NEW_VERSION}"
    git push --set-upstream origin "prepare-${NEW_VERSION}"
fi

