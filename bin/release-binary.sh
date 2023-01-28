#!/usr/bin/env bash

MONGO_URL="$1"
MONGO_USERNAME="$2"
MONGO_PASSWORD="$3"
RELEASE_VERSION="$4"

if [[ -z "$MONGO_URL" || -z "$MONGO_USERNAME" || -z "$MONGO_PASSWORD" || -z "$RELEASE_VERSION" ]]; then
  echo "Cannot release stableNativeCliVersion: $RELEASE_VERSION"
	echo "Missing parameters..."
else
  echo "Releasing stableNativeCliVersion: $RELEASE_VERSION"
  mongo "${MONGO_URL}" \
    --username="${MONGO_USERNAME}" \
    --password="${MONGO_PASSWORD}" \
    --eval "db.application.updateOne({}, {\$set: { \"stableNativeCliVersion\": \"$RELEASE_VERSION\", \"betaNativeCliVersion\": \"$RELEASE_VERSION\"}});"
fi
