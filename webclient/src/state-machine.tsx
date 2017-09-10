export type TTState = {
  stationId: string;
  stationName: string;

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
export type FinishChangeStationAction = TTAction<TTActionTypes.FINISH_CHANGE_STATION, string>;

export type TTActions = StartChangeStationAction | FinishChangeStationAction;

export const initialState: TTState = {
  stationId: "028",
  stationName: "Not loaded yet.",
  loading: false,
};

export function transition<T, P>(state: TTState = initialState, action: TTActions): TTState {
  console.log("REDUCER state.stationId = " + state.stationId);
  let partialState: Partial<TTState> | undefined;
  switch (action.type) {
  case TTActionTypes.START_CHANGE_STATION: {
    console.log("START_CHANGE_STATION -> " + action.payload);
    partialState = {
      loading: true,
      stationId: action.payload,
      stationName: "::Loading::",
    };
    break;
  }
  case TTActionTypes.FINISH_CHANGE_STATION: {
    console.log("FINISH_CHANGE_STATION -> " + action.payload);
    partialState = {
      stationName: action.payload,
      loading: false,
    };
    break;
  }
  }

  return partialState != null ? { ...state, ...partialState } : state;
}
