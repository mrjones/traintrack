import * as proto from './webclient_api_pb';

export default class DataFetcher {
  public fetchStationStatus(stationId: string): Promise<proto.StationStatus> {
    return new Promise<proto.StationStatus>((resolve: (s: proto.StationStatus) => void) => {
      const url = "/api/station/" + stationId;
      fetch(url).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationStatus = proto.StationStatus.decode(bodyBytes);
        resolve(stationStatus);
      });
    });
  }

  public fetchStationList(): Promise<proto.StationList> {
    return new Promise<proto.StationList>((resolve: (s: proto.StationList) => void) => {
      fetch("/api/stations").then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationList = proto.StationList.decode(bodyBytes);
        resolve(stationList);
      });
    });
  }
}
