```shell
export NEW_VERSION=""
git switch develop
git fetch origin
git diff origin/develop | if [ "$0" = "" ]; then
    echo "git diff found local changes on develop branch, cannot cut release."
elif [ "$NEW_VERSION" = "" ]; then
    echo "No version set. Are you just copying and pasting this without checking?"
else
    git pull origin develop --ff-only
    git switch -c "prepare-${NEW_VERSION}"
    ./update-versions.sh "${NEW_VERSION}"
    git diff # sanity check the diffs
    git commit -a -m "Update version to ${NEW_VERSION}"
    git push --set-upstream origin "prepare-${NEW_VERSION}"
fi
```

- go make a PR to `develop` on GitHub
- start "draft new release" on GitHub to ask it to "Generate release notes".  Make sure to choose the `develop` branch to get the full set of changes.: https://github.com/tcdi/pgrx/releases/new
- paste them into the PR you made above
- edit them as best as you can while channeling @workingjubilee's spirit
- request a review
- do a squash merge into develop

```shell
git switch develop
git pull origin develop --ff-only
git switch master
git pull origin master --ff-only
git merge develop
git push origin master
```

- create the actual release on GitHub, tagging the `master` branch with "${NEW_VERSION}", using the release notes you made in your PR

```shell
./publish.sh
```
