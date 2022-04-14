#!/usr/bin/env bash

echo_and_run() {
  echo $1
  echo "--------------------"
  echo "* Command to run:"
  echo "  \$ $2"
  echo ""

  eval "$2"

  echo ""
  echo ""
}
