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

import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";

import { webclient_api } from './webclient_api_pb';

import { Loadable } from './async';
import { ApiDebugger } from './debug';
import { DebuggableResult } from './datafetcher';
import { StationStats } from './recent-stations';
import { TTActionTypes, TTContext, TTState } from './state-machine';
import { fetchStationList, loadLineList } from './state-actions';
import { TTThunkDispatch } from './thunk-types';

class LinePickerDataProps {
  public dataLoaded: boolean;
  public lineList: DebuggableResult<webclient_api.LineList>;
}
class LinePickerDispatchProps {
  // TODO(mrjones): Is this the right place?
  public initializeData: () => any;
}
class LinePickerExplicitProps { }
class LinePickerLocalState { }

type LinePickerProps = LinePickerDataProps & LinePickerDispatchProps & LinePickerExplicitProps;

export default class LinePicker extends React.Component<LinePickerProps, LinePickerLocalState> {
  constructor(props: LinePickerProps) {
    super(props);
    this.state = { };
  }

  public componentDidMount() {
    this.props.initializeData();
  }

  public render(): JSX.Element {
    if (!this.props.dataLoaded) {
      return <div>Loading...</div>;
    }
    let lineLis = this.props.lineList.data.line.map((line: webclient_api.Line) => {
      let c = "#" + line.colorHex;
      let liStyle = {
        background: c,
      };

      let className = line.active ? "active" : "inactive";
      return <li key={line.name} style={liStyle} className={className}>
        <ReactRouter.Link to={`/app/line/${line.name}`}>
          {line.name}
        </ReactRouter.Link>
      </li>;
    });

    return (<div>
            <ul className="lineList">{lineLis}</ul>
            <ApiDebugger datasFetched={[this.props.lineList]} />
            </div>);
  }
}

const mapLineStateToProps = (state: TTState, ownProps: LinePickerExplicitProps): LinePickerDataProps => ({
  dataLoaded: state.core.allLines.valid,
  lineList: state.core.allLines.data,
});

const mapLineDispatchToProps = (dispatch: TTThunkDispatch): LinePickerDispatchProps => ({
  initializeData: () => dispatch(loadLineList()),
});

export let ConnectedLinePicker = ReactRedux.connect(mapLineStateToProps, mapLineDispatchToProps)(LinePicker);

export class LinePickerRouterWrapper extends React.Component<any, any> {
  constructor(props: any) {
    super(props);
  }

  public render(): JSX.Element {
    return <ConnectedLinePicker />;
  }
}

class StationPickerDataProps {
  public allStations: Loadable<webclient_api.StationList>;
}
class StationPickerDispatchProps {
  public loadStationData: () => any;
}
class StationPickerExplicitProps {
  public stationId: string;
}
type StationPickerAllProps = StationPickerDataProps & StationPickerDispatchProps & StationPickerExplicitProps;

class StationPickerLocalState {
  public currentJumpText: string;
  public currentFilterText: string;
}

const mapStationStateToProps = (state: TTState, ownProps: StationPickerExplicitProps): StationPickerDataProps => ({
  allStations: state.core.allStations,
});

const mapStationDispatchToProps = (dispatch: TTThunkDispatch): StationPickerDispatchProps => ({
  loadStationData: () => dispatch(fetchStationList()),
});

export class StationPicker extends React.Component<StationPickerAllProps, StationPickerLocalState> {
  constructor(props: StationPickerAllProps) {
    super(props);

    this.state = {
      currentJumpText: props.stationId,
      currentFilterText: "",
    };
  }

  public componentDidMount() {
    if (!this.props.allStations.valid && !this.props.allStations.loading) {
      this.props.loadStationData();
    }
  }

  private handleFilterTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentFilterText: e.currentTarget.value});
  }

  public render() {
    if (this.props.allStations.loading) {
      return <div className="stationPicker">Loading</div>;
    }
    if (!this.props.allStations.valid) {
      return <div className="stationPicker">Error.</div>;
    }
    let i = 0;
    const max = 10;
    let andMore = false;

    let matchingStationMap = new Map<string, webclient_api.Station>();
    this.props.allStations.data.station.forEach(
      (station: webclient_api.Station) => {
        if (station.name.toLowerCase().indexOf(this.state.currentFilterText.toLowerCase()) >= 0) {
          matchingStationMap.set(station.id, station);
        }
      });

    let stationsInOrder: webclient_api.Station[] = [];

    // Put recent stations first
    let stationStats = StationStats.fromCookie();
    stationStats.recentStations.forEach((stationId: string) => {
      if (i >= max) {
        andMore = true;
        return;
      }

      let maybeStation: webclient_api.Station = matchingStationMap.get(stationId);
      if (maybeStation) {
        stationsInOrder.push(maybeStation)
        matchingStationMap.delete(stationId);
        i++;
      }
    });

    // Then random other stations until we have "max"
    // TODO(mrjones): Maybe sort somehow? Alphabetically?
    matchingStationMap.forEach((station: webclient_api.Station) => {
      if (i >= max) {
        andMore = true;
        return;
      }

      stationsInOrder.push(station)
      i++;
    });

    let stationLis = stationsInOrder.map(
      (station: webclient_api.Station) => {
        return <li key={station.id}><ReactRouter.Link to={`/app/station/${station.id}`}>{station.name} ({station.lines.join(" ")})</ReactRouter.Link></li>;
      });

    if (andMore) {
      stationLis.push(<li key="andMore" className="more">And more...</li>);
    };

    return (<div className="stationPicker">
  <input autoFocus type="text" value={this.state.currentFilterText} onChange={this.handleFilterTextChanged.bind(this)} autoComplete="off" placeholder="Filter stations"/>
    <ul>{stationLis}</ul>
    </div>);
  }
}

export let ConnectedStationPicker = ReactRedux.connect(mapStationStateToProps, mapStationDispatchToProps)(StationPicker);
