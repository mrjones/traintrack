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

import { webclient_api } from './webclient_api_pb';
import { Loadable } from './async';
import { DataFetcher, DebuggableResult } from './datafetcher';
import * as Immutable from 'immutable';

export class TTContext {
  public dataFetcher: DataFetcher;

  public constructor(dataFetcher: DataFetcher) {
    this.dataFetcher = dataFetcher;
  }
}

// TODO(mrjones): split up and refactor
export type TTCoreState = {
  stationDetails: Immutable.Map<string, Loadable<DebuggableResult<webclient_api.StationStatus>>>;

  allStations: Loadable<webclient_api.StationList>,

  allLines: Loadable<DebuggableResult<webclient_api.LineList>>,
  lineDetails: Immutable.Map<string, Loadable<DebuggableResult<webclient_api.StationList>>>;

  trainItineraries: Immutable.Map<string, Loadable<DebuggableResult<webclient_api.ITrainItinerary>>>;
}

export type TTState = {
  core: TTCoreState;
}

export interface TTAction<T, P> {
  type: T;
  payload: P;
}

export enum TTActionTypes {
  START_LOADING_STATION_DETAILS = "START_LOADING_STATION_DETAILS",
  INSTALL_STATION_DETAILS = "INSTALL_STATION_DETAILS",

  INSTALL_STATION_LIST = "INSTALL_STATION_LIST",
  START_LOADING_STATION_LIST = "START_LOADING_STATION_LIST",

  INSTALL_LINE_LIST = "INSTALL_LINE_LIST",
  START_LOADING_LINE_LIST = "START_LOADING_LINE_LIST",

  INSTALL_LINE_DETAILS = "INSTALL_LINE_DETAILS",

  START_LOADING_TRAIN_ITINERARY = "START_LOADING_TRAIN_ITINERARY",
  INSTALL_TRAIN_ITINERARY = "INSTALL_TRAIN_ITINERARY",
};

export type StartLoadingStationDetailsAction = TTAction<TTActionTypes.START_LOADING_STATION_DETAILS, string>;
export type InstallStationDetailsAction = TTAction<TTActionTypes.INSTALL_STATION_DETAILS, [string, DebuggableResult<webclient_api.StationStatus>]>;

export type InstallStationListAction = TTAction<TTActionTypes.INSTALL_STATION_LIST, webclient_api.StationList>;
export type StartLoadingStationListAction = TTAction<TTActionTypes.START_LOADING_STATION_LIST, null>;

export type InstallLineListAction = TTAction<TTActionTypes.INSTALL_LINE_LIST, DebuggableResult<webclient_api.LineList>>;
export type StartLoadingLineListAction = TTAction<TTActionTypes.START_LOADING_LINE_LIST, null>;

export type InstallLineDetailsAction = TTAction<TTActionTypes.INSTALL_LINE_DETAILS, [string, DebuggableResult<webclient_api.StationList>]>;

export type StartLoadingTrainItineraryAction = TTAction<TTActionTypes.START_LOADING_TRAIN_ITINERARY, string>;
export type InstallTrainItineraryAction = TTAction<TTActionTypes.INSTALL_TRAIN_ITINERARY, [string, DebuggableResult<webclient_api.ITrainItinerary>]>;

export type TTActions =
  InstallStationDetailsAction | StartLoadingStationDetailsAction |
  InstallStationListAction | StartLoadingStationListAction |
  InstallLineListAction | StartLoadingLineListAction |
  InstallLineDetailsAction |
  InstallTrainItineraryAction | StartLoadingTrainItineraryAction;

export const initialState: TTCoreState = {
  stationDetails: Immutable.Map(),

  allStations: {loading: false, valid: false},
  allLines: {loading: false, valid: false},
  lineDetails: Immutable.Map(),

  trainItineraries: Immutable.Map(),
};

export function transition<T, P>(state: TTCoreState = initialState, action: TTActions): TTCoreState {
  let partialState: Partial<TTCoreState> | undefined;
  switch (action.type) {
  case TTActionTypes.START_LOADING_STATION_DETAILS: {
    let id: string = action.payload;
    console.log("START_LOADING_STATION_DETAILS -> " + id);
    let defaultValue: Loadable<DebuggableResult<webclient_api.StationStatus>> = {
      data: new DebuggableResult<webclient_api.StationStatus>(new webclient_api.StationStatus(), null, null),
      loading: true,
      valid: false,
    };
    let loadable: Loadable<DebuggableResult<webclient_api.StationStatus>> =
      state.stationDetails.get(id, defaultValue);
    loadable.loading = true;
    partialState = {
      stationDetails: state.stationDetails.set(id, loadable),
    };
    break;
  }
  case TTActionTypes.INSTALL_STATION_DETAILS: {
    let id: string = action.payload[0];
    let obj: Loadable<DebuggableResult<webclient_api.StationStatus>> = {
      data: action.payload[1],
      loading: false,
      valid: true,
    };
    console.log("INSTALL_STATION_DETAILS -> " + id);
    partialState = {
      stationDetails: state.stationDetails.set(id, obj),
    };
    break;
  }
  case TTActionTypes.START_LOADING_STATION_LIST: {
    console.log("START_LOADING_STATION_LIST");
    partialState = {
      allStations: state.allStations,
    };
    partialState.allStations.loading = true;
    break;

  }
  case TTActionTypes.INSTALL_STATION_LIST: {
    console.log("INSTALL_STATION_LIST");
    let obj: Loadable<webclient_api.StationList> = {
      data: action.payload,
      loading: false,
      valid: true,
    };
    partialState = {
      allStations: obj,
    };
    break;
  }
  case TTActionTypes.START_LOADING_LINE_LIST: {
    console.log("START_LOADING_LINE_LIST");
    partialState = {
      allLines: state.allLines,
    };
    partialState.allLines.loading = true;
    break;

  }
  case TTActionTypes.INSTALL_LINE_LIST: {
    console.log("INSTALL_LINE_LIST");
    partialState = {
      allLines: {loading: false, valid: true, data: action.payload},
    };
    break;
  }
  case TTActionTypes.INSTALL_LINE_DETAILS: {
    let id: string = action.payload[0];
    let obj: Loadable<DebuggableResult<webclient_api.StationList>> = {
      data: action.payload[1],
      loading: false,
      valid: true,
    };
    console.log("INSTALL_LINE_DETAILS -> " + id);
    partialState = {
      lineDetails: state.lineDetails.set(id, obj),
    };
    break;
  }
  case TTActionTypes.START_LOADING_TRAIN_ITINERARY: {
    let id: string = action.payload;
    console.log("START_LOADING_TRAIN_ITINERARY -> " + id);
    let defaultValue: Loadable<DebuggableResult<webclient_api.ITrainItinerary>> = {
      data: new DebuggableResult<webclient_api.ITrainItinerary>(new webclient_api.TrainItinerary(), null, null),
      loading: true,
      valid: false,
    };
    let loadable: Loadable<DebuggableResult<webclient_api.ITrainItinerary>> =
      state.trainItineraries.get(id, defaultValue);
    loadable.loading = true;
    partialState = {
      trainItineraries: state.trainItineraries.set(id, loadable),
    };
    break;
  }
  case TTActionTypes.INSTALL_TRAIN_ITINERARY: {
    let id: string = action.payload[0];
    let obj: Loadable<DebuggableResult<webclient_api.ITrainItinerary>> = {
      data: action.payload[1],
      loading: false,
      valid: true,
    };
    console.log("INSTALL_TRAIN_ITINERARY -> " + id);
    partialState = {
      trainItineraries: state.trainItineraries.set(id, obj),
    };
    break;
  }
  }

  return partialState != null ? { ...state, ...partialState } : state;
}
