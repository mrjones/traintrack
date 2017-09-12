import * as proto from './webclient_api_pb';

import { DebuggableResult } from './datafetcher';
import * as Immutable from 'immutable';

// TODO(mrjones): split up and refactor
export type TTCoreState = {
  currentStationId: string;
  stationDetails: Immutable.Map<string, DebuggableResult<proto.StationStatus> >;
  loading: boolean;

  mixMultipleLines: boolean;
  lineVisibility: Immutable.Map<string, boolean>;
  directionVisibility: Immutable.Map<proto.Direction, boolean>;
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
  INSTALL_STATION_DETAILS = "INSTALL_STATION_DETAILS",

  CHANGE_LINE_MIXING = "CHANGE_LINE_MIXING",
  CHANGE_LINE_VISIBILITY = "CHANGE_LINE_VISIBILITY",
  CHANGE_DIRECTION_VISIBILITY = "CHANGE_DIRECTION_VISIBILITY",
};

export type StartChangeStationAction = TTAction<TTActionTypes.START_CHANGE_STATION, string>;
export type FinishChangeStationAction = TTAction<TTActionTypes.FINISH_CHANGE_STATION, any>;
export type InstallStationDetailsAction = TTAction<TTActionTypes.INSTALL_STATION_DETAILS, [string, DebuggableResult<proto.StationStatus>]>;

export type ChangeLineMixingAction = TTAction<TTActionTypes.CHANGE_LINE_MIXING, boolean>;
export type ChangeLineVisibilityAction = TTAction<TTActionTypes.CHANGE_LINE_VISIBILITY, [string, boolean]>;
export type ChangeDirectionVisibilityAction = TTAction<TTActionTypes.CHANGE_DIRECTION_VISIBILITY, [proto.Direction, boolean]>;

export type TTActions =
  StartChangeStationAction | FinishChangeStationAction | InstallStationDetailsAction |
  ChangeLineMixingAction | ChangeLineVisibilityAction | ChangeDirectionVisibilityAction;

export const initialState: TTCoreState = {
  currentStationId: "028",
  stationDetails: Immutable.Map(),
  loading: false,

  mixMultipleLines: false,
  lineVisibility: Immutable.Map<string, boolean>(),
  directionVisibility: Immutable.Map<proto.Direction, boolean>(),
};

export function transition<T, P>(state: TTCoreState = initialState, action: TTActions): TTCoreState {
  console.log("REDUCER state.currentStationId = " + state.currentStationId);
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
  case TTActionTypes.INSTALL_STATION_DETAILS: {
    let id: string = action.payload[0];
    let data: DebuggableResult<proto.StationStatus> = action.payload[1];
    partialState = {
      stationDetails: state.stationDetails.set(id, data),
    };
    break;
  }
  case TTActionTypes.CHANGE_LINE_MIXING: {
    partialState = {
      mixMultipleLines: action.payload,
    };
    break;
  }
  case TTActionTypes.CHANGE_LINE_VISIBILITY: {
    partialState = {
      lineVisibility: state.lineVisibility.set(action.payload[0], action.payload[1]),
    };
    break;
  }
  case TTActionTypes.CHANGE_DIRECTION_VISIBILITY: {
    partialState = {
      directionVisibility: state.directionVisibility.set(action.payload[0], action.payload[1]),
    };
    break;
  }
  }

  return partialState != null ? { ...state, ...partialState } : state;
}
