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

export function loadStationDetails(stationId: string, isPrefetch = false) {
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    let existing = getState().core.stationDetails.get(stationId);
    if (existing !== undefined && existing.loading) {
      // Someone is already loading this
      // TODO(mrjone): check for errors which might wedge us in "loading"
      return;
    }
    dispatch(startLoadingStationDetails(stationId));
    context.dataFetcher.fetchStationStatus(stationId, isPrefetch)
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
