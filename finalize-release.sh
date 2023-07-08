#! /usr/bin/env bash

git switch develop
git pull origin develop --ff-only
git switch master
git pull origin master --ff-only
git merge develop
git push origin master
