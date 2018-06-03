import * as Redux from "redux";
import * as Cookie from "es-cookie";

import { TTContext, TTState } from './state-machine';
import { loadStationDetails } from './state-actions';



export class Prefetcher {
  constructor(context: TTContext, reduxStore: Redux.Store<TTState>) {
    console.log("Prefetcher ctor");
    this.context = context;
    this.reduxStore = reduxStore;
  }

  public prefetchRecentStations() {
    const recentStationsStr = Cookie.get("recentStations");
    if (recentStationsStr === undefined) { return; }


    const recentStations = recentStationsStr.split(":");
    for (let i = 0; i < 2 && i < recentStations.length; i++) {
      this.prefetchStation(recentStations[i]);
    }
  }

  private prefetchStation(stationId: string) {
    let loadFn = loadStationDetails(stationId, true);

    loadFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);
  }

  private context: TTContext;
  private reduxStore: Redux.Store<TTState>;
}
