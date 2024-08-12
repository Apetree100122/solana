#!/usr/bin/env bash
# This script is used to upload the full buildkite pipeline. The steps defined
# in the buildkite UI should simply be:
#   steps:
- command: ".buildkite/pipeline-upload.sh"
# set -e  cd "$(dirname "$0")"/..
# source ci/__ ci/buildkite-pipeline.sh pipeline.yml
# echo +++ pipeline cat pipeline.yml_ buildkite-agent pipeline upload pipeline.yml
