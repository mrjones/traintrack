import * as proto from './webclient_api_pb';

import { DataFetcher, DebuggableResult } from './datafetcher';
import * as Immutable from 'immutable';

export class TTContext {
  public dataFetcher: DataFetcher;

  public constructor(dataFetcher: DataFetcher) {
    this.dataFetcher = dataFetcher;
  }
}

export type Loadable<T> = {
  loading: boolean;
  valid: boolean;
  data: T,
}

// TODO(mrjones): split up and refactor
export type TTCoreState = {
  currentStationId: string;
  stationDetails: Immutable.Map<string, Loadable<DebuggableResult<proto.StationStatus>>>;
  loading: boolean;

  mixMultipleLines: boolean;
  lineVisibility: Immutable.Map<string, boolean>;
  directionVisibility: Immutable.Map<proto.Direction, boolean>;

  allStations: proto.StationList,
}

export type TTState = {
  core: TTCoreState;
}

export interface TTAction<T, P> {
  type: T;
  payload: P;
}

export enum TTActionTypes {
  START_CHANGE_STATION = "START_CHANGE_STATION",
  FINISH_CHANGE_STATION = "FINISH_CHANGE_STATION",
  START_LOADING_STATION_DETAILS = "START_LOADING_STATION_DETAILS",
  INSTALL_STATION_DETAILS = "INSTALL_STATION_DETAILS",

  CHANGE_LINE_MIXING = "CHANGE_LINE_MIXING",
  CHANGE_LINE_VISIBILITY = "CHANGE_LINE_VISIBILITY",
  CHANGE_DIRECTION_VISIBILITY = "CHANGE_DIRECTION_VISIBILITY",

  INSTALL_STATION_LIST = "INSTALL_STATION_LIST",
};

export type StartChangeStationAction = TTAction<TTActionTypes.START_CHANGE_STATION, string>;
export type FinishChangeStationAction = TTAction<TTActionTypes.FINISH_CHANGE_STATION, any>;
export type StartLoadingStationDetailsAction = TTAction<TTActionTypes.START_LOADING_STATION_DETAILS, string>;
export type InstallStationDetailsAction = TTAction<TTActionTypes.INSTALL_STATION_DETAILS, [string, DebuggableResult<proto.StationStatus>]>;

export type ChangeLineMixingAction = TTAction<TTActionTypes.CHANGE_LINE_MIXING, boolean>;
export type ChangeLineVisibilityAction = TTAction<TTActionTypes.CHANGE_LINE_VISIBILITY, [string, boolean]>;
export type ChangeDirectionVisibilityAction = TTAction<TTActionTypes.CHANGE_DIRECTION_VISIBILITY, [proto.Direction, boolean]>;

export type InstallStationListAction = TTAction<TTActionTypes.INSTALL_STATION_LIST, proto.StationList>;

export type TTActions =
  StartChangeStationAction | FinishChangeStationAction | InstallStationDetailsAction | StartLoadingStationDetailsAction |
  ChangeLineMixingAction | ChangeLineVisibilityAction | ChangeDirectionVisibilityAction |
  InstallStationListAction;

export const initialState: TTCoreState = {
  currentStationId: "028",
  stationDetails: Immutable.Map(),
  loading: false,

  mixMultipleLines: false,
  lineVisibility: Immutable.Map<string, boolean>(),
  directionVisibility: Immutable.Map<proto.Direction, boolean>(),

  allStations: new proto.StationList(),
};

export function transition<T, P>(state: TTCoreState = initialState, action: TTActions): TTCoreState {
  let partialState: Partial<TTCoreState> | undefined;
  switch (action.type) {
  case TTActionTypes.START_CHANGE_STATION: {
    console.log("START_CHANGE_STATION -> " + action.payload);
    partialState = {
      loading: true,
      currentStationId: action.payload,
    };
    break;
  }
  case TTActionTypes.FINISH_CHANGE_STATION: {
    console.log("FINISH_CHANGE_STATION -> " + action.payload);

    partialState = {
      loading: false,
    };
    break;
  }
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
  case TTActionTypes.CHANGE_LINE_MIXING: {
    console.log("CHANGE_LINE_MIXING -> " + action.payload);
    partialState = {
      mixMultipleLines: action.payload,
    };
    break;
  }
  case TTActionTypes.CHANGE_LINE_VISIBILITY: {
    console.log("CHANGE_LINE_VISIBILITY -> " + action.payload[0] + "::" + action.payload[1]);
    partialState = {
      lineVisibility: state.lineVisibility.set(action.payload[0], action.payload[1]),
    };
    break;
  }
  case TTActionTypes.CHANGE_DIRECTION_VISIBILITY: {
    console.log("CHANGE_DIRECTION_VISIBILITY -> " + action.payload[0] + "::" + action.payload[1]);
    partialState = {
      directionVisibility: state.directionVisibility.set(action.payload[0], action.payload[1]),
    };
    break;
  }
  case TTActionTypes.INSTALL_STATION_LIST: {
    console.log("INSTALL_STATION_LIST");
    partialState = {
      allStations: action.payload,
    };
    break;
  }
  }

  return partialState != null ? { ...state, ...partialState } : state;
}
