// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import 'babel-polyfill';

import * as moment from "moment";

import { webclient_api } from './webclient_api_pb';

import { ClientDebugInfo } from './debug';

export default class Foo {};

export enum RequestInitiator { PREFETCH, ON_DEMAND, UNKNOWN };

export class DebuggableResult<T> {
  public data: T;
  public apiUrl: string;
  public serverDebugInfo?: webclient_api.IDebugInfo;
  public clientDebugInfo?: ClientDebugInfo;
  public initiator: RequestInitiator;

  public constructor(data: T, apiUrl: string, serverDebugInfo?: webclient_api.IDebugInfo, clientDebugInfo?: ClientDebugInfo) {
    this.data = data;
    this.apiUrl = apiUrl;
    this.serverDebugInfo = serverDebugInfo;
    this.clientDebugInfo = clientDebugInfo;
    this.initiator = RequestInitiator.UNKNOWN;
  }

  public setInitiator(initiator: RequestInitiator) { this.initiator = initiator; }
}

export class DataFetcher {
  private artificialDelayMs?: number;

  public constructor(artificialDelayMs?: number) {
    console.log("new DataFetcher");
    this.artificialDelayMs = artificialDelayMs;
  }

  public fetchLineList(): Promise<DebuggableResult<webclient_api.LineList>> {
    return new Promise<DebuggableResult<webclient_api.LineList>>((resolve: (l: DebuggableResult<webclient_api.LineList>) => void) => {
      let startMoment = moment();
      let apiUrl = "/api/lines";
      fetch(apiUrl).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const lineList = webclient_api.LineList.decode(bodyBytes);

        let result = new DebuggableResult(lineList, apiUrl, lineList.debugInfo, new ClientDebugInfo(startMoment, moment()));
        this.maybeDelayAndResolve(resolve, result);
      });
    });
  }

  public fetchStationStatus(stationId: string, isPrefetch = false): Promise<DebuggableResult<webclient_api.StationStatus>> {
    return new Promise<DebuggableResult<webclient_api.StationStatus>>((resolve: (s: DebuggableResult<webclient_api.StationStatus>) => void) => {
      let startMoment = moment();
      const url = "/api/station/" + stationId + (isPrefetch ? "?prefetch=true" : "");
      fetch(url, {credentials: 'include'}).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationStatus = webclient_api.StationStatus.decode(bodyBytes);
        let result = new DebuggableResult(stationStatus, url, stationStatus.debugInfo, new ClientDebugInfo(startMoment, moment()));
        this.maybeDelayAndResolve(resolve, result);
      });
    });
  }

  public fetchStationList(): Promise<DebuggableResult<webclient_api.StationList>> {
    return new Promise<DebuggableResult<webclient_api.StationList>>((resolve: (s: DebuggableResult<webclient_api.StationList>) => void) => {
      let startMoment = moment();
      let url = "/api/stations";

      console.log("Requesting station list");
      fetch(url, {credentials: 'include'}).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationList = webclient_api.StationList.decode(bodyBytes);
        let result = new DebuggableResult(stationList, url, stationList.debugInfo, new ClientDebugInfo(startMoment, moment()));
        this.maybeDelayAndResolve(resolve, result);
      });
    });
  }

  public fetchStationsForLine(lineId: string): Promise<DebuggableResult<webclient_api.StationList>> {
    return new Promise<DebuggableResult<webclient_api.StationList>>((resolve: (s: DebuggableResult<webclient_api.StationList>) => void) => {
      let url = "/api/stations/byline/" + lineId;

      fetch(url).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const stationList = webclient_api.StationList.decode(bodyBytes);
        let result = new DebuggableResult(stationList, url, stationList.debugInfo);
        this.maybeDelayAndResolve(resolve, result);
      });
    });
  }

  public fetchTrainItinerary(trainId: string): Promise<DebuggableResult<webclient_api.TrainItinerary>> {
    return new Promise<DebuggableResult<webclient_api.TrainItinerary>>((resolve: (ti: DebuggableResult<webclient_api.TrainItinerary>) => void) => {
      let startMoment = moment();
      let url = "/api/train/" + trainId;

      fetch(url).then((response: Response) => {
        return response.arrayBuffer();
      }).then((bodyBuffer: ArrayBuffer) => {
        const bodyBytes = new Uint8Array(bodyBuffer);
        const itinerary = webclient_api.TrainItinerary.decode(bodyBytes);
        let result = new DebuggableResult(itinerary, url, itinerary.debugInfo, new ClientDebugInfo(startMoment, moment()));

        this.maybeDelayAndResolve(resolve, result);
      });
    });
  }

  private maybeDelayAndResolve<R>(
    resolve: (r: DebuggableResult<R>) => void,
    result: DebuggableResult<R>) {

    if (this.artificialDelayMs) {
      setTimeout(() => {
        resolve(result);
      }, this.artificialDelayMs);
    } else {
      resolve(result);
    }
  }
}
