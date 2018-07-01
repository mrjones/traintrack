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

import * as proto from './webclient_api_pb';

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
  stationDetails: Immutable.Map<string, Loadable<DebuggableResult<proto.StationStatus>>>;

  allStations: Loadable<proto.StationList>,

  allLines: Loadable<DebuggableResult<proto.LineList>>,
  lineDetails: Immutable.Map<string, Loadable<DebuggableResult<proto.StationList>>>;

  trainItineraries: Immutable.Map<string, Loadable<DebuggableResult<proto.ITrainItinerary>>>;
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

  INSTALL_LINE_LIST = "INSTALL_LINE_LIST",
  INSTALL_LINE_DETAILS = "INSTALL_LINE_DETAILS",

  START_LOADING_TRAIN_ITINERARY = "START_LOADING_TRAIN_ITINERARY",
  INSTALL_TRAIN_ITINERARY = "INSTALL_TRAIN_ITINERARY",
};

export type StartLoadingStationDetailsAction = TTAction<TTActionTypes.START_LOADING_STATION_DETAILS, string>;
export type InstallStationDetailsAction = TTAction<TTActionTypes.INSTALL_STATION_DETAILS, [string, DebuggableResult<proto.StationStatus>]>;

export type InstallStationListAction = TTAction<TTActionTypes.INSTALL_STATION_LIST, proto.StationList>;

export type InstallLineListAction = TTAction<TTActionTypes.INSTALL_LINE_LIST, DebuggableResult<proto.LineList>>;
export type InstallLineDetailsAction = TTAction<TTActionTypes.INSTALL_LINE_DETAILS, [string, DebuggableResult<proto.StationList>]>;

export type StartLoadingTrainItineraryAction = TTAction<TTActionTypes.START_LOADING_TRAIN_ITINERARY, string>;
export type InstallTrainItineraryAction = TTAction<TTActionTypes.INSTALL_TRAIN_ITINERARY, [string, DebuggableResult<proto.ITrainItinerary>]>;

export type TTActions =
  InstallStationDetailsAction | StartLoadingStationDetailsAction |
  InstallStationListAction |
  InstallLineListAction | InstallLineDetailsAction |
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
    let defaultValue: Loadable<DebuggableResult<proto.StationStatus>> = {
      data: new DebuggableResult<proto.StationStatus>(new proto.StationStatus(), null, null),
      loading: true,
      valid: false,
    };
    let loadable: Loadable<DebuggableResult<proto.StationStatus>> =
      state.stationDetails.get(id, defaultValue);
    loadable.loading = true;
    partialState = {
      stationDetails: state.stationDetails.set(id, loadable),
    };
    break;
  }
  case TTActionTypes.INSTALL_STATION_DETAILS: {
    let id: string = action.payload[0];
    let obj: Loadable<DebuggableResult<proto.StationStatus>> = {
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
  case TTActionTypes.INSTALL_STATION_LIST: {
    console.log("INSTALL_STATION_LIST");
    let obj: Loadable<proto.StationList> = {
      data: action.payload,
      loading: false,
      valid: true,
    };
    partialState = {
      allStations: obj,
    };
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
    let obj: Loadable<DebuggableResult<proto.StationList>> = {
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
    let defaultValue: Loadable<DebuggableResult<proto.ITrainItinerary>> = {
      data: new DebuggableResult<proto.ITrainItinerary>(new proto.TrainItinerary(), null, null),
      loading: true,
      valid: false,
    };
    let loadable: Loadable<DebuggableResult<proto.ITrainItinerary>> =
      state.trainItineraries.get(id, defaultValue);
    loadable.loading = true;
    partialState = {
      trainItineraries: state.trainItineraries.set(id, loadable),
    };
    break;
  }
  case TTActionTypes.INSTALL_TRAIN_ITINERARY: {
    let id: string = action.payload[0];
    let obj: Loadable<DebuggableResult<proto.ITrainItinerary>> = {
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
