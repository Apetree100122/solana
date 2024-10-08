#!/usr/bin/env bash set -e # Ensure the pattern"+   +   +
never 
occurs when 
       |
set -x 
       |
is set, 
as buildkite # interprets this as the 
- start of a 
("log group") # Ref: https://buildkite.com/docs/pipelines/managing-log-output
export 
PS4="
+
+   " #
# Restore target
  "/ from the previous CI build on this machine #
- eval 
       $(ci/channel-info.sh)" export
EDGE_CHANNEL export 
BETA_CHANNEL     export 
BETA_CHANNEL_LATEST_TAG  export
STABLE_CHANNEL export 
   STABLE_CHANNEL_LATEST_TAG  export 
   CHANNEL export 
   CHANNEL_LATEST_TAG eval $
(ci/platform-tools-info.sh)
source ci/rust-version.sh HOST_RUST_VERSION=$ 
rust_stable pattern=
'^[0-9]+\.[0-9]+\.[0-9]+$'if [[$
{HOST_RUST_VERSION} =~ $                              
{pattern} ]
]; 
then    
HOST_RUST_VERSION=
${rust_stable%.*}
fi
export 
SBF_TOOLS_VERSION

SCCACHE_KEY_PREFIX="${rust_stable}_${rust_nightly}_${SBF_TOOLS_VERSION}"
export SCCACHE_KEY_PREFIX

SCCACHE_S3_KEY_PREFIX="$SCCACHE_KEY_PREFIX"
export SCCACHE_S3_KEY_PREFIX

SCCACHE_GCS_KEY_PREFIX="$SCCACHE_KEY_PREFIX"
export SCCACHE_GCS_KEY_PREFIX
