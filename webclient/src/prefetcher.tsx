import * as React from "react";
import * as Redux from "redux";
import * as Cookie from "es-cookie";

import { TTContext, TTState } from './state-machine';
import { fetchStationList, loadLineList, loadStationDetails } from './state-actions';



export class Prefetcher {
  constructor(enabled: boolean, context: TTContext, reduxStore: Redux.Store<TTState>) {
    this.enabled = enabled;
    this.context = context;
    this.reduxStore = reduxStore;
  }

  public statusPage(): JSX.Element {
    return <div>Status: {this.enabled ? "Enabled" : "Disabled"}</div>;
  }

  public isEnabled(): boolean {
    return this.enabled;
  }

  public prefetchRecentStations() {
    const recentStationsStr = Cookie.get("recentStations");
    if (recentStationsStr === undefined) { return; }


    const recentStations = recentStationsStr.split(":");
    for (let i = 0; i < 2 && i < recentStations.length; i++) {
      this.prefetchStation(recentStations[i]);
    }

    let loadLineListFn = loadLineList();
    loadLineListFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);

    let loadStationListFn = fetchStationList();
    loadStationListFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);
  }

  private prefetchStation(stationId: string) {
    let loadFn = loadStationDetails(stationId, true);

    loadFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);
  }

  private enabled: boolean;
  private context: TTContext;
  private reduxStore: Redux.Store<TTState>;
}
