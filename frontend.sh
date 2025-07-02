#!/bin/bash

# Source the secrets file
. ./secrets.sh

TRAINTRACK_VERSION=DEV cargo build --color=never --bin traintrack && target/debug/traintrack \
  --mta-api-key=$MTA_API_KEY \
  --proxy-url=http://localhost:3839/ \
  --google-api-id $GOOGLE_API_ID \
  --google-api-secret $GOOGLE_API_SECRET \
  --firebase-api-key $FIREBASE_API_KEY \
  --google-service-account-pem-file $GOOGLE_SERVICE_ACCOUNT_PEM_FILE \
  --webclient-js-file ./webclient/dist/bin/webclient.js.gz \
  --webclient-js-gzipped

#  --gcs-archive-bucket traintrack-feeds-nearline-testing

