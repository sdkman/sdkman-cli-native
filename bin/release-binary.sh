#!/usr/bin/env bash

MONGO_URL="$1"
MONGO_USERNAME="$2"
MONGO_PASSWORD="$3"
RELEASE_VERSION="$4"
RELEASE_CHANNEL="$5"

if [[ -z "$MONGO_URL" || -z "$MONGO_USERNAME" || -z "$MONGO_PASSWORD" || -z "$RELEASE_VERSION" || -z "$RELEASE_CHANNEL" ]]; then
  echo "Cannot perform release!"
  echo "Missing parameters..."
else
  echo "Releasing $RELEASE_CHANNEL: $RELEASE_VERSION"
  case "$RELEASE_CHANNEL" in
    beta)   NATIVE_FIELD="betaNativeCliVersion" ;;
    stable) NATIVE_FIELD="stableNativeCliVersion" ;;
  esac
  echo "Updating field: $NATIVE_FIELD"
  docker run mongo:3.2 mongo "${MONGO_URL}" \
    --username="${MONGO_USERNAME}" \
    --password="${MONGO_PASSWORD}" \
    --quiet \
    --eval "db.application.updateOne({}, {\$set: { \"$NATIVE_FIELD\": \"$RELEASE_VERSION\"}});"
fi
