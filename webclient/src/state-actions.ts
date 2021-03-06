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

import { webclient_api } from './webclient_api_pb';

import { DebuggableResult, RequestInitiator } from './datafetcher';
import { TTActionTypes, TTContext, TTState, StartLoadingStationDetailsAction, InstallStationDetailsAction, InstallStationListAction, StartLoadingStationListAction, InstallLineListAction, StartLoadingLineListAction } from './state-machine';
import { TTThunkDispatch } from './thunk-types';

function startLoadingStationDetails(stationId: string): StartLoadingStationDetailsAction {
  return {
    type: TTActionTypes.START_LOADING_STATION_DETAILS,
    payload: stationId,
  };
}

function installStationDetails(newStationId: string, newStationInfo: DebuggableResult<webclient_api.StationStatus>): InstallStationDetailsAction {
  return {
    type: TTActionTypes.INSTALL_STATION_DETAILS,
    payload: [newStationId, newStationInfo],
  };
}

export function loadStationDetails(stationId: string, initiator = RequestInitiator.UNKNOWN) {
  return (dispatch: Redux.Dispatch, getState: () => TTState, context: TTContext) => {
    let existing = getState().core.stationDetails.get(stationId);
    if (existing !== undefined && existing.loading) {
      // Someone is already loading this
      // TODO(mrjone): check for errors which might wedge us in "loading"
      return;
    }
    dispatch(startLoadingStationDetails(stationId));
    context.dataFetcher.fetchStationStatus(stationId, initiator == RequestInitiator.PREFETCH)
      .then((result: DebuggableResult<webclient_api.StationStatus>) => {
        result.setInitiator(initiator);
        dispatch(installStationDetails(stationId, result));
      });
  };
}

export function loadMultipleStationDetails(stationIds: string[], initiator = RequestInitiator.UNKNOWN) {
  return (dispatch: Redux.Dispatch, getState: () => TTState, context: TTContext) => {
    for (let stationId of stationIds) {
      let existing = getState().core.stationDetails.get(stationId);
      if (existing !== undefined && existing.loading) {
        // Someone is already loading this
        // TODO(mrjone): check for errors which might wedge us in "loading"
        continue;
      }
      dispatch(startLoadingStationDetails(stationId));
      context.dataFetcher.fetchStationStatus(stationId)
        .then((result: DebuggableResult<webclient_api.StationStatus>) => {
          result.setInitiator(initiator);
          dispatch(installStationDetails(stationId, result));
        });
    }
  };
}

function installStationList(allStations: webclient_api.StationList): InstallStationListAction {
  return {
    type: TTActionTypes.INSTALL_STATION_LIST,
    payload: allStations,
  };
}

function startLoadingStationList(): StartLoadingStationListAction {
  return {
    type: TTActionTypes.START_LOADING_STATION_LIST,
    payload: null,
  };
}


export function fetchStationList(initiator = RequestInitiator.UNKNOWN) {
  return (dispatch: Redux.Dispatch, getState: () => TTState, context: TTContext) => {
    dispatch(startLoadingStationList());
    context.dataFetcher.fetchStationList()
      .then((result: DebuggableResult<webclient_api.StationList>) => {
        result.setInitiator(initiator);
        dispatch(installStationList(result.data));
      });
  };
}

function startLoadingLineList(): StartLoadingLineListAction {
  return {
    type: TTActionTypes.START_LOADING_LINE_LIST,
    payload: null,
  };
}

function installLineList(allLines: DebuggableResult<webclient_api.LineList>): InstallLineListAction {
  return {
    type: TTActionTypes.INSTALL_LINE_LIST,
    payload: allLines,
  };
}

export function loadLineList(initiator = RequestInitiator.UNKNOWN) {
  return (dispatch: TTThunkDispatch, getState: () => TTState, context: TTContext) => {
    if (getState().core.allLines.valid || getState().core.allLines.loading) {
      return;
    }
    dispatch(startLoadingLineList());
    context.dataFetcher.fetchLineList()
      .then((result: DebuggableResult<webclient_api.LineList>) => {
        result.setInitiator(initiator);
        dispatch(installLineList(result));
      });
  };
}
