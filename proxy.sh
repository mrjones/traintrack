 #!/bin/bash

# Source the secrets file
. ./secrets.sh

# Default values
PROXY_PORT=3839
LOG_DIR="."

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --port=*)
            PROXY_PORT="${1#*=}"
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

cargo build --color=never --bin feedproxy && 
     target/debug/feedproxy 
         --port $PROXY_PORT 
         --mta-api-key=$MTA_API_KEY 
         --use-new-mta-api-endpoint 
         --google-service-account-pem-file $GOOGLE_SERVICE_ACCOUNT_PEM_FILE 
         --gcs-archive-bucket $GCS_ARCHIVE_BUCKET 
         --log-dir $LOG_DIR

#      --mta-api-key=b596b03ace2495fe020f1dad0f67ef86 \
