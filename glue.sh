#!/usr/bin/env bash

#### Configuration ####
ROC_REPO="$HOME/build/roc/"
#######################

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir" || exit

roc glue "$ROC_REPO/crates/glue/src/RustGlue.roc" "src/glue"
