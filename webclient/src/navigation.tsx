import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";

import * as proto from './webclient_api_pb';

import { ApiDebugger } from './debug';
import { DataFetcher, DebuggableResult } from './datafetcher';
import { TTState } from './state-machine';

class LinePickerProps {
  public dataFetcher: DataFetcher;
}

class LinePickerState {
  public lineList: DebuggableResult<proto.LineList>;
}

export default class LinePicker extends React.Component<LinePickerProps, any> {
  constructor(props: LinePickerProps) {
    super(props);
    this.state = {
      lineList: new DebuggableResult<proto.LineList>(new proto.LineList(), ""),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchLineList()
      .then((res: DebuggableResult<proto.LineList>) => {
        this.setState({lineList: res});
      });
  }

  public render(): JSX.Element {
    let lineLis = this.state.lineList.data.line.map((line: proto.Line) => {
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
            <ApiDebugger datasFetched={[this.state.lineList]} />
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

class StationPickerDataProps {
  public allStations: proto.StationList;
}
class StationPickerDispatchProps { }
class StationPickerExplicitProps {
  public stationId: string;
}
type StationPickerAllProps = StationPickerDataProps & StationPickerDispatchProps & StationPickerExplicitProps;

class StationPickerLocalState {
  public currentJumpText: string;
  public currentFilterText: string;
}

const mapStateToProps = (state: TTState, ownProps: StationPickerExplicitProps): StationPickerDataProps => ({
  allStations: state.core.allStations,
});

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): StationPickerDispatchProps => ({ });

export class StationPicker extends React.Component<StationPickerAllProps, StationPickerLocalState> {
  constructor(props: StationPickerAllProps) {
    super(props);

    this.state = {
      currentJumpText: props.stationId,
      currentFilterText: "",
    };
  }

  private handleFilterTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentFilterText: e.currentTarget.value});
  }

  public render() {
    let i = 0;
    const max = 10;
    let done = false;
    let stationLis = this.props.allStations.station.map(
      (station: proto.Station) => {
        if (station.name.toLowerCase().indexOf(this.state.currentFilterText.toLowerCase()) > -1) {
          if (i++ < max && !done) {
            // TODO(mrjones): Inject the link URL for flexibility
            return <li key={station.id}><ReactRouter.Link to={`/app/station/${station.id}`}>{station.name}</ReactRouter.Link></li>;
          } else if (!done) {
            done = true;
            return <li className="more">And more...</li>;
          }
        }
      });

    return (<div className="stationPicker">
  <input type="text" value={this.state.currentFilterText} onChange={this.handleFilterTextChanged.bind(this)} autoComplete="off" placeholder="Filter stations"/>
    <ul>{stationLis}</ul>
    </div>);
  }
}

export let ConnectedStationPicker = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(StationPicker);
