#!/bin/bash

_main() {
  local checked_files=$(git diff --name-only --staged | grep \.rs)

  if [ "$checked_files" == "" ]; then
    return 0
  fi

  local unformated_files=$(
    echo $checked_files | \
      xargs cargo fmt --check --message-format short -- | \
      uniq | \
      sort
  )

  [ "$unformatted_files" == "" ]
  local has_error=$?

  if [ $has_error -eq 0 ]; then
    echo "No style issue found. Commiting!"
  else
    echo "The file(s) below has some code style issue - run 'cargo fmt' to fix them"
    echo $unformated_files
  fi

  return $has_error
}

_main
