#!/bin/bash

# Source the secrets file
. ./secrets.sh

# Default values
PROXY_PORT=3839
FRONTEND_PORT=3838
LOG_DIR="."

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --proxy_port=*)
            PROXY_PORT="${1#*=}"
            ;;
        --port=*)
            FRONTEND_PORT="${1#*=}"
            ;;
        --log_dir=*)
            LOG_DIR="${1#*=}"
            ;;
        *)
            echo "Unknown parameter passed: $1"
            exit 1
            ;;
    esac
    shift
done

TRAINTRACK_VERSION=DEV cargo build --color=never --bin traintrack && target/debug/traintrack 
  --mta-api-key=$MTA_API_KEY 
  --proxy-url=http://localhost:$PROXY_PORT/ 
  --google-api-id $GOOGLE_API_ID 
  --google-api-secret $GOOGLE_API_SECRET 
  --firebase-api-key $FIREBASE_API_KEY 
  --google-service-account-pem-file $GOOGLE_SERVICE_ACCOUNT_PEM_FILE 
  --webclient-js-file ./webclient/dist/bin/webclient.js.gz 
  --webclient-js-gzipped 
  --port $FRONTEND_PORT 
  --log-dir $LOG_DIR

#  --gcs-archive-bucket traintrack-feeds-nearline-testing





