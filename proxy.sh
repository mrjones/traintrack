 #!/bin/bash

# Source the secrets file
. ./secrets.sh

cargo build --color=never --bin feedproxy && \
     target/debug/feedproxy \
         --mta-api-key=$MTA_API_KEY \
         --use-new-mta-api-endpoint \
         --google-service-account-pem-file $GOOGLE_SERVICE_ACCOUNT_PEM_FILE \
         --gcs-archive-bucket $GCS_ARCHIVE_BUCKET

#      --mta-api-key=b596b03ace2495fe020f1dad0f67ef86 \
