```shellscript
$ export NEW_VERSION=0.6.0-alpha.2
$ git checkout develop
$ git pull
$ git checkout -b ${NEW_VERSION}-prep

$ ./upgrade-deps.sh
$ git diff     # sanity check changes
$ git commit -am "upgrade dependencies"

$ ./update-versions.sh ${NEW_VERSION}
$ git diff     # sanity check changes
$ git commit -am "update version to ${NEW_VERSION}"
$ git push -u origin ${NEW_VERSION}-prep
```

> go make a PR to `develop` on GitHub

> start "draft new release" on GitHub to ask it to "Generate release notes".  Make sure to choose the `develop` branch to get the full set of changes.: https://github.com/tcdi/pgx/releases/new

> paste them into the PR you made above

> edit them as best as you can while channeling @workingjubilee's spirit

> request a review

> do a squash merge into develop

```shellscript
$ git checkout develop
$ git pull
$ git checkout master
$ git pull
$ git merge develop
$ git push
```

> create the actual release on GitHub, tagging the `master` branch with ${NEW_VERSION}, using the release notes you made in your PR


```shellscript
$ ./publish.sh
```