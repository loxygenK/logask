#!/bin/bash

_prepare_hook() {
  if cat ./.git/hooks/pre-commit | grep "# --- prepare-hook.sh ---" 2>&1 > /dev/null; then
    echo "✅ It seems that you have already run this script!"
    return 0
  fi

  cat <<- EOF >> ./.git/hooks/pre-commit
		# --- prepare-hook.sh ---
		./.scripts/pre-commit-hook.sh
	EOF

  echo "✅ Hook registered!"

  return 0
}

_prepare_hook

# vim: tabstop=2 :
