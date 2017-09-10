import * as proto from './webclient_api_pb';

import { DebuggableResult } from './datafetcher';
import * as Immutable from 'immutable';

export type TTState = {
  currentStationId: string;

  stationDetails: Immutable.Map<string, DebuggableResult<proto.StationStatus> >;

  loading: boolean;
}

export interface TTAction<T, P> {
  type: T;
  payload: P;
}

export enum TTActionTypes {
  START_CHANGE_STATION = "START_CHANGE_STATION",
  FINISH_CHANGE_STATION = "FINISH_CHANGE_STATION",
};

export type StartChangeStationAction = TTAction<TTActionTypes.START_CHANGE_STATION, string>;
export type FinishChangeStationAction = TTAction<TTActionTypes.FINISH_CHANGE_STATION, [string, DebuggableResult<proto.StationStatus>]>;

export type TTActions = StartChangeStationAction | FinishChangeStationAction;

export const initialState: TTState = {
  currentStationId: "028",
  stationDetails: Immutable.Map(),
  loading: false,
};

export function transition<T, P>(state: TTState = initialState, action: TTActions): TTState {
  console.log("REDUCER state.currentStationId = " + state.currentStationId);
  let partialState: Partial<TTState> | undefined;
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

    let id: string = action.payload[0];
    let data: DebuggableResult<proto.StationStatus> = action.payload[1];
    let newDetails = state.stationDetails.set(id, data);
    partialState = {
      stationDetails: newDetails,
      loading: false,
    };
    break;
  }
  }

  return partialState != null ? { ...state, ...partialState } : state;
}
