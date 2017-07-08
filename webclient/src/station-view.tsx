import * as moment from "moment";
import * as React from "react";
import * as ReactRouter from "react-router-dom";
import * as proto from './webclient_api_pb';

import { DataFetcher, DebuggableResult } from './datafetcher';
import { ApiDebugger, ApiRequestInfo } from './debug';
import { StationPicker } from './navigation';

class StationLineProps {
  public data: proto.LineArrivals;
};

class StationLine extends React.Component<StationLineProps, undefined> {
  constructor(props: StationLineProps) {
    super(props);
  };

  public render() {
    const arrivals = this.props.data.arrivals.map(
      (arr: proto.LineArrival) => {
        const ts = arr.timestamp as number;
        const time = moment.unix(ts);

        if (time > moment()) {
          return <li key={ts}><ReactRouter.Link to={`/app/train/${arr.tripId}`}>{time.format("LT")}</ReactRouter.Link> ({time.fromNow()})</li>;
        } else {
          return <li key={ts}><s>{time.format("LT")}</s></li>;
        }
      }
    );

    let lineStyle = {
      //      borderBottom: "2px solid #" + this.props.data.lineColorHex,
      background: "#" + this.props.data.lineColorHex,
    };

    return (
      <div className="stationLine">
        <div className="header" style={lineStyle}>{this.props.data.line} - {directionName(this.props.data.direction)}</div>
        <ul>{arrivals}</ul>
      </div>);
  }
};

class StationBoardState {
  public stationId: string;
  public stationName: string;
  public data: DebuggableResult<proto.StationStatus>;
};

class StationBoardProps {
  public stationId: string;
  public dataFetcher: DataFetcher;
}

class StationBoard extends React.Component<StationBoardProps, StationBoardState> {
  constructor(props: StationBoardProps) {
    super(props);
    this.state = {
      stationId: "",
      stationName: "Loading...",
      data: new DebuggableResult(new proto.StationStatus(), "unknown"),
    };
  }

  public componentDidMount() {
    this.stationChanged();
  }

  public componentDidUpdate() {
    if (this.props.stationId !== this.state.stationId) {
      this.stationChanged();
    }
  }

  private stationChanged() {
    const component = this;
    this.props.dataFetcher
      .fetchStationStatus(this.props.stationId)
      .then((stationStatus: DebuggableResult<proto.StationStatus>) => {
        component.setState({
          stationId: component.props.stationId,
          stationName: stationStatus.data.name,
          data: stationStatus,
        });
      });
  }

  public render() {
    const lineSet = this.state.data.data.line.map(
      (line: proto.LineArrivals) => {
        const key = this.props.stationId + "-" + line.line + "-" + line.direction;
        return <StationLine data={line} key={key} />;
      });

    const dataTs = moment.unix(this.state.data.data.dataTimestamp as number);

    return (<div className="stationInfo">
            <h2>{this.state.stationName}</h2>
            <div className="pubTime">Published at {dataTs.format("LTS")} ({dataTs.fromNow()}) <a href="#" onClick={this.stationChanged.bind(this)}>Reload</a></div>
            {lineSet}
            <ApiDebugger requestInfo={new ApiRequestInfo(this.state.data.apiUrl, this.state.data.serverDebugInfo, this.state.data.clientDebugInfo)}/>
            </div>);
  };
}

class OneStationViewProps {
  public initialStationId: string;
  public dataFetcher: DataFetcher;
}

class OneStationViewState {
  public stationIdAtLoad: string;
  public displayedStationId: string;
  public stationPickerDisplayed: boolean;
};

export class OneStationView extends React.Component<OneStationViewProps, OneStationViewState> {
  constructor(props: OneStationViewProps) {
    super(props);

    console.log("OneStationView.ctor. props.initial=" + props.initialStationId);

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
    console.log("OneStationView.componentDidUpdate: BEFORE " +
                "props.initial=" + this.props.initialStationId +
                ", state.atLoad="  + this.state.stationIdAtLoad +
                ", state.displayed=" + this.state.displayedStationId);
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
      <StationBoard stationId={this.state.displayedStationId} dataFetcher={this.props.dataFetcher}/>
    </div>);
  }
}

export class OneStationViewWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  private dataFetcher: DataFetcher;

  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
    this.dataFetcher = globalDataFetcher;

    console.log("OneStationViewWrapper.ctor: " + JSON.stringify(this.props.match));
    this.state = {
      stationId: this.props.match.params.initialStationId ? this.props.match.params.initialStationId : "R32",
    };
  }

  public componentDidMount() {
    console.log("OneStationViewWrapper.componentDidMount: " + JSON.stringify(this.props.match));
  }

  public componentDidUpdate() {
    console.log("OneStationViewWrapper.componentDidUpdate: " + JSON.stringify(this.props.match));
    let newStation = this.props.match.params.initialStationId ? this.props.match.params.initialStationId : "R32";
    if (newStation !== this.state.stationId) {
      this.setState({stationId: newStation});
    }
  }

  public render() {
    return <OneStationView initialStationId={this.state.stationId} dataFetcher={this.dataFetcher} />;
  }
}

function directionName(direction: proto.Direction): string {
  switch (direction) {
    case proto.Direction.UPTOWN:
      return "Uptown";
    case proto.Direction.DOWNTOWN:
      return "Downtown";
    default:
      console.log("Unknown Direction: " + direction);
      return "" + direction;
  }
}

let globalDataFetcher = new DataFetcher();
