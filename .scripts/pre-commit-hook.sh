#!/bin/bash

_main() {
  local checked_files=$(git diff --name-only --staged | grep \.rs)

  if [ "$checked_files" == "" ]; then
    return 0
  fi

  local unformatted_files=$(
    echo $checked_files | \
      xargs cargo fmt --check --message-format short -- | \
      sort | \
      uniq
  )

  [ "$unformatted_files" == "" ]
  local has_error=$?

  if [ $has_error -eq 0 ]; then
    echo "No style issue found. Commiting!"
  else
    echo "The file(s) below has some code style issue - run 'cargo fmt' to fix them"
    for file in $unformatted_files; do
      echo $file
    done
  fi

  return $has_error
}

_main
