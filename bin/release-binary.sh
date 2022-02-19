#!/usr/bin/env bash

MONGO_URL="$1"
MONGO_USERNAME="$2"
MONGO_PASSWORD="$3"
RELEASE_VERSION="$4"

echo "Mongo URL: $MONGO_URL"

if [[ -z "$MONGO_USERNAME" || -z "$MONGO_PASSWORD" ]]; then
	echo "No mongo credentials so doing nothing..."
	return 1
fi

if [[ -z "$RELEASE_VERSION" ]]; then
	echo "No release version set..."
	return 1
fi

echo "Release: stableNativeCliVersion as $RELEASE_VERSION"

mongo "${MONGO_URL}" \
  --username="${MONGO_USERNAME}" \
  --password="${MONGO_PASSWORD}" \
  --eval "db.application.updateOne({}, {\$set: { \"stableNativeCliVersion\": \"$RELEASE_VERSION\"}});"
