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

import * as Immutable from 'immutable';
import * as moment from "moment";
import * as querystring from "query-string";
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as history from "history";

import * as proto from './webclient_api_pb';
import * as utils from './utils';

import { Loadable } from './async';
import { DebuggableResult, RequestInitiator } from './datafetcher';
import { ApiDebugger } from './debug';
import { ConnectedFilterControl, VisibilityState } from './filter-control';
import { ConnectedStationPicker } from './navigation';
import { PubInfo } from './pub-info';
import { TTState } from './state-machine';
import { loadStationDetails } from './state-actions';
import { TTThunkDispatch } from './thunk-types';

export class StationPageQueryParams {
  public static parseFrom(query: history.Search): StationPageQueryParams {
    let parsed = querystring.parse(query);
    return {
      highlightedTrains: parsed["highlight"] ?
        Immutable.Set(parsed["highlight"].split(",")) : Immutable.Set(),
    };
  }

  public highlightedTrains: Immutable.Set<string>;
}

class StationIntermingledLineProps {
  public data: proto.ILineArrivals[];
  public stationId: string;
  public highlightedTrains: Immutable.Set<string>;
}

class IntermingledArrivalInfo {
  public line: string;
  public timestamp: number | Long;
  public direction: proto.Direction;
  public tripId: string;
  public lineColorHex: string;
}

class StationIntermingledLines extends React.Component<StationIntermingledLineProps, undefined> {
  public render(): JSX.Element {
    let directionUls = new Array<JSX.Element>();
    let directionDatas = this.sortArrivals(this.props.data);
    console.log("HIGHLIGHTS: " + JSON.stringify(this.props.highlightedTrains));
    for (let directionData of directionDatas) {
      let lis = directionData[1].map((info: IntermingledArrivalInfo) => {
        let key = info.line + "-" + info.timestamp + "-" + info.direction;
        const time = moment.unix(info.timestamp as number);
        let style = {
          background: "#" + info.lineColorHex,
        };

        let className = "upcoming";
        if (time < moment()) {
          className = "expired";
        }

        if (this.props.highlightedTrains.has(info.tripId)) {
          className = className + " highlighted";
        }

        return <li key={key} className={className}><span className="lineName" style={style}>{info.line}</span> <ReactRouter.Link to={`/app/train/${info.tripId}?highlight=${this.props.stationId}`}>{time.format("LT")}</ReactRouter.Link> ({time.fromNow()})</li>;
      });
      directionUls.push(<div className="intermingledArrivals"><div className="header">{utils.directionName(directionData[0])}</div><ul>{lis}</ul></div>);
    }

    return <div>{directionUls}</div>;
  }

  private sortArrivals(arrivals: proto.ILineArrivals[]): Map<proto.Direction, IntermingledArrivalInfo[]> {
    let infoMap = new Map<proto.Direction, IntermingledArrivalInfo[]>();
    arrivals.map((oneLine: proto.ILineArrivals) => {
      oneLine.arrivals.map((oneArrival: proto.LineArrival) => {
        let info = new IntermingledArrivalInfo();
        info.line = oneLine.line;
        info.direction = oneLine.direction;
        info.timestamp = oneArrival.timestamp;
        info.tripId = oneArrival.tripId;
        info.lineColorHex = oneLine.lineColorHex;

        if (!infoMap.has(oneLine.direction)) {
          infoMap.set(oneLine.direction, new Array<IntermingledArrivalInfo>());
        }
        infoMap.get(oneLine.direction).push(info);
      });
    });

    for (let info of infoMap) {
      info[1].sort((a: IntermingledArrivalInfo, b: IntermingledArrivalInfo) => {
        if (a.timestamp < b.timestamp) { return -1; }
        if (a.timestamp > b.timestamp) { return 1; }
        return 0;
      });
    }
    return infoMap;
  }
};

class StationSingleLineProps {
  public data: proto.LineArrivals;
  public stationId: string;
  public highlightedTrains: Immutable.Set<string>;
};

class StationSingleLine extends React.Component<StationSingleLineProps, undefined> {
  constructor(props: StationSingleLineProps) {
    super(props);
  };

  public render() {
    const arrivals = this.props.data.arrivals.map(
      (arr: proto.LineArrival) => {
        const ts = arr.timestamp as number;
        const time = moment.unix(ts);

        let className = "upcoming";
        if (time < moment()) {
          className = "expired";
        }
        return <li key={ts} className={className}><ReactRouter.Link to={`/app/train/${arr.tripId}?highlight=${this.props.stationId}`}>{time.format("LT")}</ReactRouter.Link> ({time.fromNow()})</li>;
      }
    );

    let lineStyle = {
      background: "#" + this.props.data.lineColorHex,
    };

    return (
      <div className="stationLine">
        <div className="header" style={lineStyle}>{this.props.data.line} - {utils.directionName(this.props.data.direction)}</div>
        <ul>{arrivals}</ul>
      </div>);
  }
};

class StationMultiLineExplicitProps {
  public stationId: string;
  public visibilityState: VisibilityState;
  public highlightedTrains: Immutable.Set<string>;
}
class StationMultiLineDataProps {
  public stationName: string;
  public data: DebuggableResult<proto.StationStatus>;
  public hasData: boolean;
  public loading: boolean;
}
class StationMultiLineDispatchProps {
  public fetchStationData: (stationId: string) => any;
}
class StationMultiLineLocalState { }

type StationMultiLineProps = StationMultiLineExplicitProps & StationMultiLineDataProps & StationMultiLineDispatchProps;

const mapStateToProps = (state: TTState, ownProps: StationMultiLineExplicitProps): StationMultiLineDataProps => {
  let maybeData: Loadable<DebuggableResult<proto.StationStatus>> =
    state.core.stationDetails.get(ownProps.stationId);
  if (maybeData !== undefined && maybeData.valid) {
    return {
      stationName: maybeData.data.data.name,
      data: maybeData.data,
      hasData: true,
      loading: maybeData.loading,
    };
  } else {
    return {
      stationName: "Loading...",
      data: new DebuggableResult<proto.StationStatus>(new proto.StationStatus(), null, null),
      hasData: false,
      loading: true,
    };
  }
};

const mapDispatchToProps = (dispatch: TTThunkDispatch): StationMultiLineDispatchProps => ({
  fetchStationData: (stationId: string) => dispatch(loadStationDetails(stationId, RequestInitiator.ON_DEMAND)),
});

class StationMultiLine extends React.Component<StationMultiLineProps, StationMultiLineLocalState> {
  constructor(props: StationMultiLineProps) {
    super(props);
    this.state = { };
  }

  private fetchDataIfNecessary(props: StationMultiLineProps) {
    if (!props.hasData) {
      props.fetchStationData(props.stationId);
    }
  }

  private fetchData() {
    this.props.fetchStationData(this.props.stationId);
  }

  public componentWillMount() {
    this.fetchDataIfNecessary(this.props);
  }

  public componentWillReceiveProps(nextProps: StationMultiLineProps) {
    this.fetchDataIfNecessary(nextProps);
  }

  public render() {
    let lineSet: JSX.Element[];
    let visibleLines = this.props.data.data.line.filter(
      (line: proto.LineArrivals) => {
        return this.props.visibilityState.includesLine(line.line) &&
          this.props.visibilityState.includesDirection(line.direction);
      });

    if (!this.props.visibilityState.isCombined()) {
      lineSet = visibleLines.map(
        (line: proto.LineArrivals) => {
          const key = this.props.stationId + "-" + line.line + "-" + line.direction;
          return <StationSingleLine data={line} key={key} stationId={this.props.stationId} highlightedTrains={this.props.highlightedTrains}/>;
        });
    } else {
      lineSet = new Array<JSX.Element>();
      lineSet.push(<StationIntermingledLines data={visibleLines} key="mixed" stationId={this.props.stationId} highlightedTrains={this.props.highlightedTrains}/>);
    }

    const dataTs = moment.unix(this.props.data.data.dataTimestamp as number);

    return (<div className="stationInfo">
            <h2>{this.props.stationName}</h2>
            <PubInfo reloadFn={this.fetchData.bind(this)} pubTimestamp={dataTs} isLoading={this.props.loading}/>
            <ConnectedFilterControl stationId={this.props.stationId} visibilityState={this.props.visibilityState}/>
            {lineSet}
            <ApiDebugger datasFetched={[this.props.data]}/>
            </div>);
  };
}

export let ConnectedStationMultiLine = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(StationMultiLine);

class StationPageProps {
  public initialStationId: string;
  public visibilityState: VisibilityState;
  public queryParams: StationPageQueryParams;
}

class StationPageState {
  public stationPickerDisplayed: boolean;
};

export class StationPage extends React.Component<StationPageProps, StationPageState> {
  constructor(props: StationPageProps) {
    super(props);

    this.state = {
      stationPickerDisplayed: false,
    };
  }

  private toggleStationPicker() {
    this.setState({stationPickerDisplayed: !this.state.stationPickerDisplayed});
  }

  public render() {
    let stationPicker;
    let stationPickerToggle;
    let className = "";
    if (this.state.stationPickerDisplayed) {
      stationPicker = <ConnectedStationPicker stationId={this.props.initialStationId} />;
      stationPickerToggle = <a href="#" onClick={this.toggleStationPicker.bind(this)}>Hide stations</a>;
      className = "jumpLink open";
    } else {
      stationPickerToggle = <a href="#" onClick={this.toggleStationPicker.bind(this)}>Jump to station</a>;
      className = "jumpLink closed";
    }
    return (<div>
      <div className="jumpLink"><ReactRouter.Link to={`/app/lines`}>Pick by line</ReactRouter.Link></div>
      <div className={className}>{stationPickerToggle}</div>
      {stationPicker}
      <ConnectedStationMultiLine stationId={this.props.initialStationId} visibilityState={this.props.visibilityState} highlightedTrains={this.props.queryParams.highlightedTrains}/>
    </div>);
  }
}

export class StationPageWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  private stationId(): string {
    return this.props.match.params.initialStationId ? this.props.match.params.initialStationId : "default";
  }

  private visibilitySpec(): string {
    return this.props.match.params.visibilitySpec ? this.props.match.params.visibilitySpec : "";
  }

  public render() {
    return <div>
      <StationPage initialStationId={this.stationId()} visibilityState={VisibilityState.parseFromSpec(this.visibilitySpec())} queryParams={StationPageQueryParams.parseFrom(this.props.location.search)}/>
    </div>;
  }
}
