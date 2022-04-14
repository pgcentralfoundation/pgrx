#!/usr/bin/env bash

echo_and_run() {
  echo $1; shift;
  echo "--------------------"
  echo "* Command to run:"
  echo "  \$ $@"
  echo ""

  eval "$@"

  echo ""
  echo ""
}
