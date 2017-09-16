import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";

import * as proto from './webclient_api_pb';

import { ApiDebugger } from './debug';
import { DebuggableResult } from './datafetcher';
import { TTActionTypes, TTContext, TTState, InstallLineListAction } from './state-machine';

class LinePickerDataProps {
  public dataLoaded: boolean;
  public lineList: DebuggableResult<proto.LineList>;
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
    let lineLis = this.props.lineList.data.line.map((line: proto.Line) => {
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

function installLineList(allLines: DebuggableResult<proto.LineList>): InstallLineListAction {
  return {
    type: TTActionTypes.INSTALL_LINE_LIST,
    payload: allLines,
  };
}

function loadLineList() {
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    if (getState().core.allLines.valid || getState().core.allLines.loading) {
      return;
    }
    context.dataFetcher.fetchLineList()
      .then((result: DebuggableResult<proto.LineList>) => {
        dispatch(installLineList(result));
      });
  };
}

const mapLineStateToProps = (state: TTState, ownProps: LinePickerExplicitProps): LinePickerDataProps => ({
  dataLoaded: state.core.allLines.valid,
  lineList: state.core.allLines.data,
});

const mapLineDispatchToProps = (dispatch: Redux.Dispatch<TTState>): LinePickerDispatchProps => ({
  initializeData: () => dispatch(loadLineList()),
});

export let ConnectedLinePicker = ReactRedux.connect(mapLineStateToProps, mapLineDispatchToProps)(LinePicker);

export class LinePickerRouterWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render(): JSX.Element {
    return <ConnectedLinePicker />;
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

const mapStationStateToProps = (state: TTState, ownProps: StationPickerExplicitProps): StationPickerDataProps => ({
  allStations: state.core.allStations,
});

const mapStationDispatchToProps = (dispatch: Redux.Dispatch<TTState>): StationPickerDispatchProps => ({ });

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

export let ConnectedStationPicker = ReactRedux.connect(mapStationStateToProps, mapStationDispatchToProps)(StationPicker);
