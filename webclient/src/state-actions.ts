import * as Redux from "redux";

import * as proto from './webclient_api_pb';

import { DebuggableResult } from './datafetcher';
import { TTActionTypes, TTContext, TTState, StartLoadingStationDetailsAction, InstallStationDetailsAction, InstallStationListAction } from './state-machine';

function startLoadingStationDetails(stationId: string): StartLoadingStationDetailsAction {
  return {
    type: TTActionTypes.START_LOADING_STATION_DETAILS,
    payload: stationId,
  };
}

function installStationDetails(newStationId: string, newStationInfo: DebuggableResult<proto.StationStatus>): InstallStationDetailsAction {
  return {
    type: TTActionTypes.INSTALL_STATION_DETAILS,
    payload: [newStationId, newStationInfo],
  };
}

export function loadStationDetails(stationId: string) {
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    let existing = getState().core.stationDetails.get(stationId);
    if (existing !== undefined && existing.loading) {
      // Someone is already loading this
      // TODO(mrjone): check for errors which might wedge us in "loading"
      return;
    }
    dispatch(startLoadingStationDetails(stationId));
    context.dataFetcher.fetchStationStatus(stationId)
      .then((result: DebuggableResult<proto.StationStatus>) => {
        dispatch(installStationDetails(stationId, result));
      });
  };
}

export function loadMultipleStationDetails(stationIds: string[]) {
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    for (let stationId of stationIds) {
      let existing = getState().core.stationDetails.get(stationId);
      if (existing !== undefined && existing.loading) {
        // Someone is already loading this
        // TODO(mrjone): check for errors which might wedge us in "loading"
        continue;
      }
      dispatch(startLoadingStationDetails(stationId));
      context.dataFetcher.fetchStationStatus(stationId)
        .then((result: DebuggableResult<proto.StationStatus>) => {
          dispatch(installStationDetails(stationId, result));
        });
    }
  };
}

function installStationList(allStations: proto.StationList): InstallStationListAction {
  return {
    type: TTActionTypes.INSTALL_STATION_LIST,
    payload: allStations,
  };
}

export function fetchStationList() {
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    context.dataFetcher.fetchStationList()
      .then((result: DebuggableResult<proto.StationList>) => {
        dispatch(installStationList(result.data));
      });
  };
}
