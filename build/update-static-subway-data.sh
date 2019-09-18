tmpdir=$(mktemp -d)
tmpzip=${tmpdir}/mta-gtfs-data.zip
echo "Downloading GTFS files";
wget -O ${tmpzip} http://web.mta.info/developers/data/nyct/subway/google_transit.zip
echo "Unzipping GTFS files"
unzip ${tmpzip} -d ${tmpdir}
echo "Downloading Stations.csv";
wget -O ${tmpdir}/Stations.csv http://web.mta.info/developers/data/nyct/subway/Stations.csv

cp ${tmpdir}/* data/
