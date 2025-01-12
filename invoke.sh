#!/usr/bin/env bash

NUMBER=${1:-}
echo $NUMBER: $(date)
cargo lambda invoke --data-file test/test-payload.json
echo $NUMBER: $(date)
