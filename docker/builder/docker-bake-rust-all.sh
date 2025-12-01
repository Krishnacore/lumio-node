#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0

# This script docker bake to build all the rust-based docker images
# You need to execute this from the repository root as working directory
# E.g. docker/docker-bake-rust-all.sh
# If you want to build a specific target only, run:
#   docker/docker-bake-rust-all.sh <target>
# See targets in docker/builder/docker-bake-rust-all.hcl
#   e.g. docker/docker-bake-rust-all.sh forge-images

set -ex

export GIT_SHA=${CI_COMMIT_SHA:-$(git rev-parse HEAD)}
export GIT_BRANCH=${CI_COMMIT_REF_NAME:-$(git symbolic-ref --short HEAD 2>/dev/null || echo "detached-${GIT_SHA:0:8}")}
export GIT_TAG=${CI_COMMIT_TAG:-$(git tag -l --contains HEAD)}
export GIT_CREDENTIALS="${GIT_CREDENTIALS:-}"
export BUILD_DATE="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
export BUILT_VIA_BUILDKIT="true"
export TARGET_CACHE_ID=${TARGET_CACHE_ID:-${GIT_BRANCH:-main}}
export NORMALIZED_GIT_BRANCH_OR_PR=$(printf "$TARGET_CACHE_ID" | sed -e 's/[^a-zA-Z0-9]/-/g')

export PROFILE=${PROFILE:-release}
export FEATURES=${FEATURES:-""}
export NORMALIZED_FEATURES_LIST=$(printf "$FEATURES" | sed -e 's/[^a-zA-Z0-9]/_/g')
export CUSTOM_IMAGE_TAG_PREFIX=${CUSTOM_IMAGE_TAG_PREFIX:-""}
export CARGO_TARGET_DIR="target/${FEATURES:-"default"}"
export TARGET_REGISTRY=${TARGET_REGISTRY:-harbor}
export HARBOR_REGISTRY=${HARBOR_REGISTRY:-registry.wings.toys/lumio/lumio-node}

# Convenience for local builds: Load SSH key if not provided in env
if [ -z "$SSH_KEY_B64" ]; then
  SSH_KEY_RAW=""
  if [ -n "$SSH_KEY" ]; then
     SSH_KEY_RAW="$SSH_KEY"
  elif [ -f "$HOME/.ssh/id_ed25519" ]; then
    echo "Loading SSH_KEY from $HOME/.ssh/id_ed25519"
    SSH_KEY_RAW=$(cat "$HOME/.ssh/id_ed25519")
  elif [ -f "$HOME/.ssh/id_rsa" ]; then
    echo "Loading SSH_KEY from $HOME/.ssh/id_rsa"
    SSH_KEY_RAW=$(cat "$HOME/.ssh/id_rsa")
  fi
  
  if [ -n "$SSH_KEY_RAW" ]; then
    # Base64 encode the key to preserve format through env vars
    # Try openssl first as it's consistent across macOS/Linux
    if command -v openssl >/dev/null; then
      export SSH_KEY_B64=$(echo "$SSH_KEY_RAW" | openssl base64 | tr -d '\n')
    else
      # Fallback to base64 command
      export SSH_KEY_B64=$(echo "$SSH_KEY_RAW" | base64 | tr -d '\n')
    fi
  else
    echo "WARNING: SSH_KEY not set and no default key found. Build might fail if private repos are needed."
    export SSH_KEY_B64=""
  fi
fi

if [ "$PROFILE" = "release" ]; then
  # Do not prefix image tags if we're building the default profile "release"
  profile_prefix=""
else
  # Builds for profiles other than "release" should be tagged with their profile name
  profile_prefix="${PROFILE}_"
fi

if [ -n "$CUSTOM_IMAGE_TAG_PREFIX" ]; then
  export IMAGE_TAG_PREFIX="${CUSTOM_IMAGE_TAG_PREFIX}_"
else
  export IMAGE_TAG_PREFIX=""
fi

if [ -n "$FEATURES" ]; then
  export IMAGE_TAG_PREFIX="${IMAGE_TAG_PREFIX}${profile_prefix}${NORMALIZED_FEATURES_LIST}_"
else
  export IMAGE_TAG_PREFIX="${IMAGE_TAG_PREFIX}${profile_prefix}"
fi

BUILD_TARGET="${1:-all}"
echo "Building target: ${BUILD_TARGET}"
echo "To build only a specific target, run: docker/builder/docker-bake-rust-all.sh <target>"
echo "E.g. docker/builder/docker-bake-rust-all.sh forge-images"

# Only use --allow=ssh if SSH_AUTH_SOCK is available (SSH agent is running)
SSH_ALLOW=""
if [ -n "$SSH_AUTH_SOCK" ]; then
  SSH_ALLOW="--allow=ssh"
fi

if [ "$CI" == "true" ]; then
  docker buildx bake $SSH_ALLOW --no-cache --pull --progress=plain --file docker/builder/docker-bake-rust-all.hcl --push $BUILD_TARGET
else
  docker buildx bake $SSH_ALLOW --no-cache --pull --file docker/builder/docker-bake-rust-all.hcl $BUILD_TARGET --load
fi

echo "Build complete. Docker buildx cache usage:"
docker buildx du --verbose --filter type=exec.cachemount
