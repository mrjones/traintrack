import * as proto from './webclient_api_pb';

class Cached<T> {
  public valid: boolean;
  public value: T;

  public constructor() {
    this.valid = false;
  }

  public set(value: T): void {
    this.value = value;
  }

  public clear(): void {
    this.valid = false;
  }
}

export default class DataFetcher {
  private stationCache: Cached<proto.StationList>;

  public constructor() {
  }

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
      if (this.stationCache.valid) {
        resolve(this.stationCache.value);
      }

      fetch("/api/stations").then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationList = proto.StationList.decode(bodyBytes);
        this.stationCache.set(stationList);
        resolve(stationList);
      });
    });
  }

}
