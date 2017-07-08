import * as React from "react";
import * as ReactRouter from "react-router-dom";

import * as proto from './webclient_api_pb';

import { ApiDebugger, ApiRequestInfo } from './debug';
import { DataFetcher, DebuggableResult } from './datafetcher';

class LinePickerProps {
  public dataFetcher: DataFetcher;
}

class LinePickerState {
  public lineList: proto.LineList;
  public apiUrl: String;
}

export default class LinePicker extends React.Component<LinePickerProps, any> {
  constructor(props: LinePickerProps) {
    super(props);
    this.state = {
      lineList: new proto.LineList(),
      apiInfo: new ApiRequestInfo("", null),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchLineList()
      .then((res: DebuggableResult<proto.LineList>) => {
        this.setState({lineList: res.data, apiInfo: new ApiRequestInfo(res.apiUrl, res.serverDebugInfo)});
      });
  }

  public render(): JSX.Element {
    let lineLis = this.state.lineList.line.map((line: proto.Line) => {
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
            <ApiDebugger requestInfo={this.state.apiInfo} />
            </div>);
  }
}

export class LinePickerRouterWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render(): JSX.Element {
    return <LinePicker dataFetcher={new DataFetcher()}/>;
  }
}

class StationPickerState {
  public currentJumpText: string;
  public currentFilterText: string;
  public allStations: proto.StationList;
};

class StationPickerProps {
  public initialStationId: string;
  public stationPickedFn: (newStation: string) => void;
  public dataFetcher: DataFetcher;
}

export class StationPicker extends React.Component<StationPickerProps, StationPickerState> {
  constructor(props: StationPickerProps) {
    super(props);

    this.state = {
      currentJumpText: props.initialStationId,
      currentFilterText: "",
      allStations: new proto.StationList(),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchStationList().then((stationList: DebuggableResult<proto.StationList>) => {
      this.setState({allStations: stationList.data});
    });
  }

  private handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    for (let station of this.state.allStations.station) {
      if (station.id === this.state.currentJumpText) {
        console.log("Matched: " + station.name);
      }
    }
    this.props.stationPickedFn(this.state.currentJumpText);
  }

  private handleCurrentTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentJumpText: e.currentTarget.value});
  }

  private handleFilterTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentFilterText: e.currentTarget.value});
  }

  public render() {
    let i = 0;
    const max = 10;
    let done = false;
    let stationLis = this.state.allStations.station.map(
      (station: proto.Station) => {
        if (station.name.toLowerCase().indexOf(this.state.currentFilterText.toLowerCase()) > -1) {
          if (i++ < max && !done) {
            // TODO(mrjones): Inject the link URL for flexibility
            return <li key={station.id}><ReactRouter.Link to={`/app/station/${station.id}`}>{station.name}</ReactRouter.Link></li>;
          } else if (!done) {
            done = true;
            return <li>...</li>;
          }
        }
      });

    return (<div className="stationPicker">
  <form onSubmit={this.handleSubmit.bind(this)}>
  </form>
  <input type="text" value={this.state.currentFilterText} onChange={this.handleFilterTextChanged.bind(this)} autoComplete="off" placeholder="Filter stations"/>
  <ul>{stationLis}</ul>
    <hr/>
    <input id="stationIdBox" type="text" value={this.state.currentJumpText} onChange={this.handleCurrentTextChanged.bind(this)} autoComplete="off"/>
    <input type="submit" value="Jump (by ID)"/>
    </div>);
  }
}
