import 'babel-polyfill';
import * as proto from './webclient_api_pb';

export class Cached<T> {
  public valid: boolean;
  public value: T;

  public constructor() {
    this.valid = false;
  }

  public set(value: T): void {
    this.valid = true;
    this.value = value;
  }

  public clear(): void {
    this.valid = false;
  }
}

export default class Foo {};

export class DebuggableResult<T> {
  public data: T;
  public apiUrl: String;

  public constructor(data: T, apiUrl: String) {
    this.data = data;
    this.apiUrl = apiUrl;
  }
}

export class DataFetcher {
  private stationCache: Cached<proto.StationList>;
  private lineListCache: Cached<proto.LineList>;
  private stationsByLineCache: Map<string, Cached<proto.StationList>>;

  public constructor() {
    console.log("new DataFetcher");
    this.stationCache = new Cached<proto.StationList>();
    this.lineListCache = new Cached<proto.LineList>();
    this.stationsByLineCache = new Map<string, Cached<proto.StationList>>();
  }

  public fetchLineList(): Promise<DebuggableResult<proto.LineList>> {
    return new Promise<DebuggableResult<proto.LineList>>((resolve: (l: DebuggableResult<proto.LineList>) => void) => {
      let apiUrl = "/api/lines";
      if (this.stationCache.valid) {
        resolve(new DebuggableResult(this.lineListCache.value, apiUrl));
        return;
      } else {
        fetch(apiUrl).then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const lineList = proto.LineList.decode(bodyBytes);
          this.lineListCache.set(lineList);
          resolve(new DebuggableResult(lineList, apiUrl));
        });
      }
    });
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
        console.log("Using cached station list");
        resolve(this.stationCache.value);
        return;
      } else {
        console.log("Requesting station list");
        fetch("/api/stations").then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const stationList = proto.StationList.decode(bodyBytes);
          this.stationCache.set(stationList);
          resolve(stationList);
        });
      }
    });
  }

  public fetchStationsForLine(lineId: string): Promise<proto.StationList> {
    return new Promise<proto.StationList>((resolve: (s: proto.StationList) => void) => {
      if (!this.stationsByLineCache.has(lineId)) {
        this.stationsByLineCache.set(lineId, new Cached<proto.StationList>());
      }

      let cached = this.stationsByLineCache.get(lineId);

      if (cached.valid) {
        resolve(cached.value);
        return;
      } else {
        fetch("/api/stations/byline/" + lineId).then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const stationList = proto.StationList.decode(bodyBytes);
          cached.set(stationList);
          resolve(stationList);
        });
      }
    });
  }
}
