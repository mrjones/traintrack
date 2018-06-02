import * as Redux from "redux";

import { TTContext, TTState } from './state-machine';
import { loadStationDetails } from './state-actions';

export class Prefetcher {
  constructor(context: TTContext, reduxStore: Redux.Store<TTState>) {
    console.log("Prefetcher ctor");
    this.context = context;
    this.reduxStore = reduxStore;
  }

  private prefetchStation(stationId: string) {
    let loadFn = loadStationDetails("617");

    loadFn(
      this.reduxStore.dispatch,
      this.reduxStore.getState,
      this.context);
  }

  private context: TTContext;
  private reduxStore: Redux.Store<TTState>;
}
