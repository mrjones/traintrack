import * as React from "react";
import * as Redux from "redux";
import * as Cookie from "es-cookie";

import * as proto from './webclient_api_pb';

import { Loadable } from './async';
import { DebuggableResult, RequestInitiator } from './datafetcher';
import { TTContext, TTState } from './state-machine';
import { fetchStationList, loadLineList, loadStationDetails } from './state-actions';

export class Prefetcher {
  constructor(enabled: boolean, context: TTContext, reduxStore: Redux.Store<TTState>) {
    this.enabled = enabled;
    this.context = context;
    this.reduxStore = reduxStore;
  }

  public statusPage(): JSX.Element {
    let stationLis = this.reduxStore.getState().core.stationDetails.map(
      (value: Loadable<DebuggableResult<proto.StationStatus>>, key: string) => {
        if (value.valid) {
          return <li>"{value.data.data.name}" [{key}] {RequestInitiator[value.data.initiator]}</li>;
        } else {
          return <li>Station {key} loading...</li>;
        }
      }
    ).toSet();

    return <div>
      Status: {this.enabled ? "Enabled" : "Disabled"}
      <div>Stations: <ul>{stationLis}</ul></div>
      </div>;
  }

  public isEnabled(): boolean {
    return this.enabled;
  }

  public prefetchRecentStations() {
    const recentStationsStr = Cookie.get("recentStations");
    if (recentStationsStr === undefined) { return; }


    const recentStations = recentStationsStr.split(":").reverse();
    // TODO(mrjones): Send one batch request
    for (let i = 0; i < 3 && i < recentStations.length; i++) {
      this.prefetchStation(recentStations[i]);
    }

    let loadLineListFn = loadLineList(RequestInitiator.PREFETCH);
    loadLineListFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);

    let loadStationListFn = fetchStationList(RequestInitiator.PREFETCH);
    loadStationListFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);
  }

  private prefetchStation(stationId: string) {
    let loadFn = loadStationDetails(stationId, RequestInitiator.PREFETCH);

    loadFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);
  }

  private enabled: boolean;
  private context: TTContext;
  private reduxStore: Redux.Store<TTState>;
}
