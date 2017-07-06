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
  public apiUrl: string;
  public debugInfo?: proto.IDebugInfo;

  public constructor(data: T, apiUrl: string, debugInfo?: proto.IDebugInfo) {
    this.data = data;
    this.apiUrl = apiUrl;
    this.debugInfo = debugInfo;
  }
}

export class DataFetcher {
  private stationCache: Cached<proto.StationList>;
  private lineListCache: Cached<proto.LineList>;
  private stationsByLineCache: Map<string, Cached<proto.StationList>>;
  private trainsByIdCache: Map<string, Cached<proto.TrainItinerary>>;

  public constructor() {
    console.log("new DataFetcher");
    this.stationCache = new Cached<proto.StationList>();
    this.lineListCache = new Cached<proto.LineList>();
    this.stationsByLineCache = new Map<string, Cached<proto.StationList>>();
    this.trainsByIdCache = new Map<string, Cached<proto.TrainItinerary>>();
  }

  public fetchLineList(): Promise<DebuggableResult<proto.LineList>> {
    return new Promise<DebuggableResult<proto.LineList>>((resolve: (l: DebuggableResult<proto.LineList>) => void) => {
      let apiUrl = "/api/lines";
      if (this.stationCache.valid) {
        resolve(new DebuggableResult(this.lineListCache.value, apiUrl, this.lineListCache.value.debugInfo));
        return;
      } else {
        fetch(apiUrl).then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const lineList = proto.LineList.decode(bodyBytes);
          this.lineListCache.set(lineList);
          resolve(new DebuggableResult(lineList, apiUrl, lineList.debugInfo));
        });
      }
    });
  }

  public fetchStationStatus(stationId: string): Promise<DebuggableResult<proto.StationStatus>> {
    return new Promise<DebuggableResult<proto.StationStatus>>((resolve: (s: DebuggableResult<proto.StationStatus>) => void) => {
      const url = "/api/station/" + stationId;
      fetch(url).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationStatus = proto.StationStatus.decode(bodyBytes);
        resolve(new DebuggableResult(stationStatus, url, stationStatus.debugInfo));
      });
    });
  }

  public fetchStationList(): Promise<DebuggableResult<proto.StationList>> {
    return new Promise<DebuggableResult<proto.StationList>>((resolve: (s: DebuggableResult<proto.StationList>) => void) => {
      let url = "/api/stations";

      if (this.stationCache.valid) {
        console.log("Using cached station list");
        resolve(new DebuggableResult(this.stationCache.value, url, this.stationCache.value.debugInfo));
        return;
      } else {
        console.log("Requesting station list");
        fetch(url).then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const stationList = proto.StationList.decode(bodyBytes);
          this.stationCache.set(stationList);
          resolve(new DebuggableResult(stationList, url, stationList.debugInfo));
        });
      }
    });
  }

  public fetchStationsForLine(lineId: string): Promise<DebuggableResult<proto.StationList>> {
    return new Promise<DebuggableResult<proto.StationList>>((resolve: (s: DebuggableResult<proto.StationList>) => void) => {
      let url = "/api/stations/byline/" + lineId;

      if (!this.stationsByLineCache.has(lineId)) {
        this.stationsByLineCache.set(lineId, new Cached<proto.StationList>());
      }

      let cached = this.stationsByLineCache.get(lineId);

      if (cached.valid) {
        resolve(new DebuggableResult(cached.value, url, cached.value.debugInfo));
        return;
      } else {
        fetch(url).then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const stationList = proto.StationList.decode(bodyBytes);
          cached.set(stationList);
          resolve(new DebuggableResult(stationList, url, stationList.debugInfo));
        });
      }
    });
  }

  public fetchTrainItinerary(trainId: string): Promise<DebuggableResult<proto.TrainItinerary>> {
    return new Promise<DebuggableResult<proto.TrainItinerary>>((resolve: (ti: DebuggableResult<proto.TrainItinerary>) => void) => {
      let url = "/api/train/" + trainId;

      if (!this.trainsByIdCache.has(trainId)) {
        this.trainsByIdCache.set(trainId, new Cached<proto.TrainItinerary>());
      }

      let cached = this.trainsByIdCache.get(trainId);

      if (cached.valid) {
        resolve(new DebuggableResult(cached.value, url, cached.value.debugInfo));
        return;
      } else {
        fetch(url).then((response: Response) => {
          return response.arrayBuffer();
        }).then((bodyBuffer: ArrayBuffer) => {
          const bodyBytes = new Uint8Array(bodyBuffer);
          const itinerary = proto.TrainItinerary.decode(bodyBytes);
          cached.set(itinerary);
          resolve(new DebuggableResult(itinerary, url, itinerary.debugInfo));
        });
      }
    });
  }
}
