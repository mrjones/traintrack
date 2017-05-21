import * as History from "history";
import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactRouter from "react-router-dom";
import * as moment from "moment";
import * as proto from './webclient_api_pb';

class StationPickerState {
  currentJumpText: string;
  currentFilterText: string;
  allStations: proto.StationList;
};

class StationPickerProps {
  initialStationId: string;
  stationPickedFn: (newStation: string) => void;
}

class StationPicker extends React.Component<StationPickerProps, StationPickerState> {
  constructor(props: StationPickerProps) {
    super(props);

    this.state = {
      currentJumpText: props.initialStationId,
      currentFilterText: "",
      allStations: new proto.StationList(),
    }
  }

  componentDidMount() {
    fetch("/api/stations").then((response: Response) => {
      return response.arrayBuffer();
    }).then((bodyBuffer: ArrayBuffer) => {
      const bodyBytes = new Uint8Array(bodyBuffer);
      const stationList = proto.StationList.decode(bodyBytes);
      this.setState({allStations: stationList});
    });
  }

  handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    for (let station of this.state.allStations.station) {
      if (station.id == this.state.currentJumpText) {
        console.log("Matched: " + station.name);
      }
    }
    this.props.stationPickedFn(this.state.currentJumpText);
  }

  handleCurrentTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentJumpText: e.currentTarget.value});
  }

  handleFilterTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentFilterText: e.currentTarget.value});
  }

  stationClicked(station: proto.Station,
                 e: React.FormEvent<HTMLAnchorElement>) {
    e.preventDefault();
    this.props.stationPickedFn(station.id);
  }

  render() {
    let i = 0;
    const max = 10;
    let done = false;
    let stationLis = this.state.allStations.station.map(
      (station: proto.Station) => {
        if (station.name.toLowerCase().indexOf(this.state.currentFilterText.toLowerCase()) > -1) {
          if (i++ < max && !done) {
//            return <li key={station.id}><a href="#" onClick={this.stationClicked.bind(this, station)}>{station.name}</a></li>;
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
  render() {
    return <div>
      <p>ROUTER PROXY HACK: {this.props.match.params.initialStationId}</p>
      <OneStationView initialStationId={this.props.match.params.initialStationId} />
    </div>
  }
}

class OneStationViewProps {
  initialStationId: string;
}

class OneStationViewState {
  stationIdAtLoad: string;
  displayedStationId: string;
};

class OneStationView extends React.Component<OneStationViewProps, OneStationViewState> {
  constructor(props: OneStationViewProps) {
    super(props);

    this.state ={
      stationIdAtLoad: props.initialStationId,
      displayedStationId: props.initialStationId,
    };
  }

  handleStationChanged(newStationId: string) {
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

  render() {
    return (<div>
      <StationPicker initialStationId={this.props.initialStationId} stationPickedFn={this.handleStationChanged.bind(this)} />
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
  data: proto.LineArrivals;
};

class StationLine extends React.Component<StationLineProps, undefined> {
  constructor(props: StationLineProps) {
    super(props);
  };

  render() {
    const arrivals = this.props.data.timestamp.map(
      (ts: number) => {
        const time = moment.unix(ts);

        let timeNode;
        if (time > moment()) {
          return <li key={ts}>{time.format("LT")}</li>;
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

interface StationBoardState {
  stationId: string;
  stationName: string;
  data: proto.StationStatus;
};

interface StationBoardProps {
  stationId: string;
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
    if (this.props.stationId != this.state.stationId) {
      this.stationChanged();
    }
  }

  private stationChanged() {
    const xhr = new XMLHttpRequest();
    const component = this;
    xhr.onreadystatechange = function() {
      console.log("ajax response");
      if (this.readyState === 4 && this.status === 200) {
        /*
        console.log("Deserializing: " + this.responseText);
        const a = proto.LineArrivals.deserializeBinary(this.responseText);
         */

        const bytes = new Uint8Array(xhr.response);
        //        const data = proto.StationStatus.deserializeBinary(bytes);
        const data = proto.StationStatus.decode(bytes);

        console.log("XXX Fetched station: [" + data.name + "]");
        component.setState({
          stationId: component.props.stationId,
          stationName: data.name,
          data: data,
        });
      }
    };
    console.log("Fetching " + this.props.stationId);
    xhr.responseType = 'arraybuffer';
    xhr.open("GET", "/api/station/" + this.props.stationId);
    xhr.send();
  }

  public render() {
    var lineSet = this.state.data.line.map(
      (line: proto.LineArrivals) => {
        const key = this.props.stationId + "-" + line.line + "-" + line.direction;
        return <StationLine data={line} key={key} />
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
      <ReactRouter.Route path='/singlepage/station/:initialStationId' component={OneStationViewWrapperForRouter} />
      <ReactRouter.Route path='/singlepage/'>
        <OneStationView initialStationId="R32" />
      </ReactRouter.Route>
    </ReactRouter.Switch>
  </ReactRouter.BrowserRouter>,
  document.getElementById('tt_app'));
