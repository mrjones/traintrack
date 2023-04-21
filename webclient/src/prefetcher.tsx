import * as React from "react";
import * as Redux from "redux";

import { webclient_api } from './webclient_api_pb';
import { Loadable } from './async';
import { DebuggableResult, RequestInitiator } from './datafetcher';
import { recentStationsFromCookie } from './recent-stations';
import { TTContext, TTState } from './state-machine';
import { fetchStationList, loadLineList, loadStationDetails } from './state-actions';

export class Prefetcher {
  constructor(enabled: boolean, context: TTContext, reduxStore: Redux.Store<TTState>) {
    this.enabled = enabled;
    this.context = context;
    this.reduxStore = reduxStore;
  }


  public kick(): void {
    this.prefetchMissingData();
  }

  public statusPage(): JSX.Element {
    let stationLis = this.reduxStore.getState().core.stationDetails.map(
      (value: Loadable<DebuggableResult<webclient_api.StationStatus>>, key: string) => {
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
    this.prefetchMissingData();
  }

  private prefetchMissingData() {
    const currentState = this.reduxStore.getState();

    this.prefetchStations(recentStationsFromCookie().slice(0, 3), currentState);

    if (!currentState.core.allLines.valid &&
        !currentState.core.allLines.loading) {
      console.log("Prefetching line metadata.");
      let loadLineListFn = loadLineList(RequestInitiator.PREFETCH);
      loadLineListFn(
        this.reduxStore.dispatch,
        this.reduxStore.getState,
        this.context);
    }

    if (!currentState.core.allStations.valid &&
        !currentState.core.allStations.loading) {
      console.log("Prefetching station list.");;
      let loadStationListFn = fetchStationList(RequestInitiator.PREFETCH);
      loadStationListFn(
        this.reduxStore.dispatch,
        this.reduxStore.getState,
        this.context);
    }
  }

  private prefetchStations(stationIds: string[], state: TTState) {
    // TODO(mrjones): Send one batch request
    stationIds.forEach((id: string) => this.prefetchStation(id, state));
  }

  private prefetchStation(stationId: string, state: TTState) {
    if (state.core.stationDetails.has(stationId)) {
      return;
    }
    console.log("Prefetching recent station: " + stationId);
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
