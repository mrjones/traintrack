import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactRouter from "react-router-dom";
import * as moment from "moment";
import * as proto from './webclient_api_pb';

import { DataFetcher } from './datafetcher';
import { LinePickerRouterWrapper } from './navigation';
import { LineViewRouterWrapper } from './lineview';

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

class StationPicker extends React.Component<StationPickerProps, StationPickerState> {
  constructor(props: StationPickerProps) {
    super(props);

    this.state = {
      currentJumpText: props.initialStationId,
      currentFilterText: "",
      allStations: new proto.StationList(),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchStationList().then((stationList: proto.StationList) => {
      this.setState({allStations: stationList});
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
            return <li key={station.id}><ReactRouter.Link to={`/singlepage/station/${station.id}`}>{station.name}</ReactRouter.Link></li>;
          } else if (!done) {
            done = true;
            return <li>...</li>;
          }
        }
      });

    return (<div>
  <form onSubmit={this.handleSubmit.bind(this)}>
    <input id="stationIdBox" type="text" value={this.state.currentJumpText} onChange={this.handleCurrentTextChanged.bind(this)} autoComplete="off"/>
    <input type="submit" value="Jump"/>
  </form>
  <input type="text" value={this.state.currentFilterText} onChange={this.handleFilterTextChanged.bind(this)} autoComplete="off" placeholder="Filter"/>
  <ul>{stationLis}</ul>
    </div>);
  }
}

class OneStationViewWrapperForRouter extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  private dataFetcher: DataFetcher;

  constructor() {
    super();
    this.dataFetcher = new DataFetcher();
  }

  public render() {
    return <div>
      <OneStationView initialStationId={this.props.match.params.initialStationId} dataFetcher={this.dataFetcher} />
    </div>
  }
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

class OneStationView extends React.Component<OneStationViewProps, OneStationViewState> {
  constructor(props: OneStationViewProps) {
    super(props);

    this.state ={
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
    if (this.state.stationPickerDisplayed) {
      stationPicker = <StationPicker initialStationId={this.props.initialStationId} stationPickedFn={this.handleStationChanged.bind(this)} dataFetcher={this.props.dataFetcher} />;
      stationPickerToggle = <a href="#" onClick={this.toggleStationPicker.bind(this)}>Hide picker</a>
    } else {
      stationPickerToggle = <a href="#" onClick={this.toggleStationPicker.bind(this)}>Pick another station</a>
    }
    return (<div>
      <div><ReactRouter.Link to={`/singlepage/lines`}>Browse Lines</ReactRouter.Link></div>
      <div>{stationPickerToggle}</div>
      {stationPicker}
      <StationBoard stationId={this.state.displayedStationId} />
    </div>);
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

class StationLineProps {
  public data: proto.LineArrivals;
};

class StationLine extends React.Component<StationLineProps, undefined> {
  constructor(props: StationLineProps) {
    super(props);
  };

  public render() {
    const arrivals = this.props.data.timestamp.map(
      (ts: number) => {
        const time = moment.unix(ts);

        if (time > moment()) {
          return <li key={ts}>{time.format("LT")} ({time.fromNow()})</li>;
        } else {
          return <li key={ts}><s>{time.format("LT")}</s></li>;
        }
      }
    );

    return (
      <div>
        <b>{this.props.data.line} - {directionName(this.props.data.direction)}</b>
        <ul>{arrivals}</ul>
      </div>);
  }
};

class StationBoardState {
  public stationId: string;
  public stationName: string;
  public data: proto.StationStatus;
};

class StationBoardProps {
  public stationId: string;
}

class StationBoard extends React.Component<StationBoardProps, StationBoardState> {
  constructor(props: StationBoardProps) {
    super(props);
    this.state = {
      stationId: "",
      stationName: "Loading...",
      data: new proto.StationStatus(),
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
    let fetcher = new DataFetcher();
    fetcher
      .fetchStationStatus(this.props.stationId)
      .then((stationStatus: proto.StationStatus) => {
        component.setState({
          stationId: component.props.stationId,
          stationName: stationStatus.name,
          data: stationStatus,
        });
      });
  }

  public render() {
    const lineSet = this.state.data.line.map(
      (line: proto.LineArrivals) => {
        const key = this.props.stationId + "-" + line.line + "-" + line.direction;
        return <StationLine data={line} key={key} />;
      });

    return (<div>
        <hr/>
        <dl>
          <dt>ID</dt>
          <dd>{this.props.stationId}</dd>
          <dt>Name</dt>
          <dd>{this.state.stationName}</dd>
        </dl>
  {lineSet}
    </div>);
  };
}

ReactDOM.render(
  <ReactRouter.BrowserRouter>
    <ReactRouter.Switch>
      <ReactRouter.Route path='/singlepage/lines' component={LinePickerRouterWrapper}/>
      <ReactRouter.Route path='/singlepage/line/:lineId' component={LineViewRouterWrapper}/>
      <ReactRouter.Route path='/singlepage/station/:initialStationId' component={OneStationViewWrapperForRouter} />
      <ReactRouter.Route path='/singlepage/'>
        <OneStationView initialStationId="R32" dataFetcher={new DataFetcher()}/>
      </ReactRouter.Route>
    </ReactRouter.Switch>
  </ReactRouter.BrowserRouter>,
  document.getElementById('tt_app'));
