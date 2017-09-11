import * as moment from "moment";
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as proto from './webclient_api_pb';

import * as utils from './utils';

import { DataFetcher, DebuggableResult } from './datafetcher';
import { ApiDebugger } from './debug';
import { FilterControl } from './filter-control';
import { StationPicker } from './navigation';
import { PubInfo } from './pub-info';

import { TTState } from './state-machine';

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

enum MultipleLineMixing { SEPARATE, INTERMINGLED };

class StationMultiLineExplicitProps {
  public dataFetcher: DataFetcher;
}
class StationMultiLineStateProps {
  public stationId: string;
  public stationName: string;
  public data: DebuggableResult<proto.StationStatus>;
}
class StationMultiLineDispatchProps { }
class StationMultiLineLocalState {
  public filterPredicate: (l: proto.LineArrivals) => boolean;
  public mixing: MultipleLineMixing;
}

const mapStateToProps = (state: TTState, ownProps: StationMultiLineExplicitProps): StationMultiLineStateProps => {
  if (state.stationDetails.has(state.currentStationId)) {
    let details: DebuggableResult<proto.StationStatus> =
      state.stationDetails.get(state.currentStationId);
    return {
      stationId: state.currentStationId,
      stationName: details.data.name,
      data: details,
    };
  } else {
    return {
      stationId: state.currentStationId,
      stationName: state.loading ? "Loading..." : "No data for station: " + state.currentStationId,
      data: new DebuggableResult<proto.StationStatus>(new proto.StationStatus(), null, null),
    };
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): StationMultiLineDispatchProps => ({});

class StationMultiLine extends React.Component<StationMultiLineStateProps & StationMultiLineDispatchProps & StationMultiLineExplicitProps, StationMultiLineLocalState> {
  constructor(props: any) {
    super(props);
    this.state = {
      filterPredicate: (l: proto.LineArrivals) => { return true; },
      mixing: MultipleLineMixing.SEPARATE,
    };
  }

  private stationChanged() {
    // TODO(mrjones): move to redux
  }

  private updateFilterPredicate(p: (l: proto.LineArrivals) => boolean) {
    this.setState({filterPredicate: p}
    );
  }

  private updateLineMixingState(mixMultipleLines: boolean) {
    this.setState({mixing: mixMultipleLines ? MultipleLineMixing.INTERMINGLED : MultipleLineMixing.SEPARATE});

  }

  public render() {
    let lineSet: JSX.Element[];
//    let visibleLines = this.state.data.data.line.filter(this.state.filterPredicate.bind(this));
    let visibleLines = this.props.data.data.line.filter(this.state.filterPredicate.bind(this));

    if (this.state.mixing === MultipleLineMixing.SEPARATE) {
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
            <PubInfo reloadFn={this.stationChanged.bind(this)} pubTimestamp={dataTs} />
            <FilterControl updateFilterPredicateFn={this.updateFilterPredicate.bind(this)} updateMixingFn={this.updateLineMixingState.bind(this)} allTrains={this.props.data.data} />
            {lineSet}
            <ApiDebugger datasFetched={[this.props.data]}/>
            </div>);
  };
}

export let ConnectedStationMultiLine = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(StationMultiLine);

class StationPageProps {
  public initialStationId: string;
  public dataFetcher: DataFetcher;
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
      stationPicker = <StationPicker initialStationId={this.props.initialStationId} stationPickedFn={this.handleStationChanged.bind(this)} dataFetcher={this.props.dataFetcher} />;
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
      <ConnectedStationMultiLine dataFetcher={this.props.dataFetcher}/>
    </div>);
  }
}

export class StationPageWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  private dataFetcher: DataFetcher;

  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
    this.dataFetcher = globalDataFetcher;

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
    return <StationPage initialStationId={this.state.stationId} dataFetcher={this.dataFetcher} />;
  }
}


let globalDataFetcher = new DataFetcher();
