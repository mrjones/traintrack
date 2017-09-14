import * as Immutable from 'immutable';
import * as moment from "moment";
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as proto from './webclient_api_pb';

import * as utils from './utils';

import { DebuggableResult } from './datafetcher';
import { ApiDebugger } from './debug';
import { ConnectedFilterControl } from './filter-control';
import { ConnectedStationPicker } from './navigation';
import { PubInfo } from './pub-info';

import { TTActionTypes, TTContext, TTState, StartLoadingStationDetailsAction, InstallStationDetailsAction, InstallStationListAction, Loadable } from './state-machine';

// TODO(mrjones): Move this somewhere better
function installStationList(allStations: proto.StationList): InstallStationListAction {
  return {
    type: TTActionTypes.INSTALL_STATION_LIST,
    payload: allStations,
  };
}

function fetchStationList() {
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    context.dataFetcher.fetchStationList()
      .then((result: DebuggableResult<proto.StationList>) => {
        dispatch(installStationList(result.data));
      });
  };
}

class StationIntermingledLineProps {
  public data: proto.ILineArrivals[];
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

        return <li key={key} className={className}><span className="lineName" style={style}>{info.line}</span> <ReactRouter.Link to={`/app/train/${info.tripId}`}>{time.format("LT")}</ReactRouter.Link> ({time.fromNow()})</li>;
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
        return <li key={ts} className={className}><ReactRouter.Link to={`/app/train/${arr.tripId}`}>{time.format("LT")}</ReactRouter.Link> ({time.fromNow()})</li>;
      }
    );

    let lineStyle = {
      //      borderBottom: "2px solid #" + this.props.data.lineColorHex,
      background: "#" + this.props.data.lineColorHex,
    };

    return (
      <div className="stationLine">
        <div className="header" style={lineStyle}>{this.props.data.line} - {utils.directionName(this.props.data.direction)}</div>
        <ul>{arrivals}</ul>
      </div>);
  }
};

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

function loadStationDetails(stationId: string) {
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

enum MultipleLineMixing { SEPARATE, INTERMINGLED };

class StationMultiLineExplicitProps {
  public stationId: string;
}
class StationMultiLineStateProps {
  public stationName: string;
  public data: DebuggableResult<proto.StationStatus>;
  public mixing: MultipleLineMixing;
  public lineVisibility: Immutable.Map<string, boolean>;
  public directionVisibility: Immutable.Map<proto.Direction, boolean>;
  public hasData: boolean;
}
class StationMultiLineDispatchProps {
  public fetchStationData: (stationId: string) => any;
  // TODO(mrjones): This belongs on StationPage (or higher) once that component is Redux-ified
  public initializeData: () => any;
}
class StationMultiLineLocalState { }

type StationMultiLineProps = StationMultiLineExplicitProps & StationMultiLineStateProps & StationMultiLineDispatchProps;

const mapStateToProps = (state: TTState, ownProps: StationMultiLineExplicitProps): StationMultiLineStateProps => {
  let maybeData: Loadable<DebuggableResult<proto.StationStatus>> =
    state.core.stationDetails.get(ownProps.stationId);
  if (maybeData !== undefined && maybeData.valid) {
    return {
      stationName: maybeData.data.data.name,
      data: maybeData.data,
      mixing: state.core.mixMultipleLines ? MultipleLineMixing.INTERMINGLED : MultipleLineMixing.SEPARATE,
      lineVisibility: state.core.lineVisibility,
      directionVisibility: state.core.directionVisibility,
      hasData: true,
    };
  } else {
    return {
      stationName: "Loading...",
      data: new DebuggableResult<proto.StationStatus>(new proto.StationStatus(), null, null),
      mixing: state.core.mixMultipleLines ? MultipleLineMixing.INTERMINGLED : MultipleLineMixing.SEPARATE,
      lineVisibility: state.core.lineVisibility,
      directionVisibility: state.core.directionVisibility,
      hasData: false,
    };
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): StationMultiLineDispatchProps => ({
  fetchStationData: (stationId: string) => dispatch(loadStationDetails(stationId)),
  initializeData: () => dispatch(fetchStationList()),
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
    // TODO(mrjones): batch?
    this.fetchDataIfNecessary(this.props);
    this.props.initializeData();
  }

  public componentWillReceiveProps(nextProps: StationMultiLineProps) {
    this.fetchDataIfNecessary(nextProps);
  }

  public render() {
    let lineSet: JSX.Element[];
    let visibleLines = this.props.data.data.line.filter((line: proto.LineArrivals) => utils.lineVisible(line, this.props.lineVisibility, this.props.directionVisibility));

    if (this.props.mixing === MultipleLineMixing.SEPARATE) {
      lineSet = visibleLines.map(
        (line: proto.LineArrivals) => {
          const key = this.props.stationId + "-" + line.line + "-" + line.direction;
          return <StationSingleLine data={line} key={key} />;
        });
    } else {
      lineSet = new Array<JSX.Element>();
      lineSet.push(<StationIntermingledLines data={visibleLines} key="mixed"/>);
    }

    const dataTs = moment.unix(this.props.data.data.dataTimestamp as number);

    return (<div className="stationInfo">
            <h2>{this.props.stationName}</h2>
            <PubInfo reloadFn={this.fetchData.bind(this)} pubTimestamp={dataTs} />
            <ConnectedFilterControl />
            {lineSet}
            <ApiDebugger datasFetched={[this.props.data]}/>
            </div>);
  };
}

export let ConnectedStationMultiLine = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(StationMultiLine);

class StationPageProps {
  public initialStationId: string;
}

class StationPageState {
  public stationIdAtLoad: string;
  public displayedStationId: string;
  public stationPickerDisplayed: boolean;
};

export class StationPage extends React.Component<StationPageProps, StationPageState> {
  constructor(props: StationPageProps) {
    super(props);

    this.state = {
      stationIdAtLoad: props.initialStationId,
      displayedStationId: props.initialStationId,
      stationPickerDisplayed: false,
    };
  }

  private handleStationChanged(newStationId: string) {
    this.setState({displayedStationId: newStationId});
  }

  public componentDidUpdate() {
    if (this.props.initialStationId != this.state.stationIdAtLoad) {
      this.setState({
        stationIdAtLoad: this.props.initialStationId,
        displayedStationId: this.props.initialStationId,
      });
    }
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
      <ConnectedStationMultiLine stationId={this.props.initialStationId}/>
    </div>);
  }
}

export class StationPageWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);

    this.state = {
      stationId: this.props.match.params.initialStationId ? this.props.match.params.initialStationId : "028",
    };
  }

  public componentDidUpdate() {
    let newStation = this.props.match.params.initialStationId ? this.props.match.params.initialStationId : "028";
    if (newStation !== this.state.stationId) {
      this.setState({stationId: newStation});
    }
  }

  public render() {
    return <StationPage initialStationId={this.state.stationId}  />;
  }
}
