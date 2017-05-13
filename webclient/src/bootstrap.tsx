import * as React from "react";
import * as ReactDOM from "react-dom";
import * as moment from "moment";
import * as proto from './webclient_api_pb';

class OneStationViewState {
  displayedStationId: string;
  inputStationId: string;
};

class OneStationView extends React.Component<any, OneStationViewState> {
  constructor(props: any) {
    super(props);

    this.state ={
      displayedStationId: "R20",
      inputStationId: "R20",
    };
  }

  handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    console.log("XXX HandleSubmit: " + this.state.inputStationId);
    this.setState({displayedStationId: this.state.inputStationId});
  }

  handleStationTextChanged(e: React.FormEvent<HTMLInputElement>) {
    console.log("XXX HandleTextChanged");
    const newId = e.currentTarget.value
    this.setState({inputStationId: newId});
  }

  render() {
    return (<div>
  <form onSubmit={this.handleSubmit.bind(this)}>
    <input id="stationIdBox" type="text" value={this.state.inputStationId} onChange={this.handleStationTextChanged.bind(this)} />
    <input type="submit" />
  </form>
  <span>Station is: {this.state.displayedStationId}</span>
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
  <div>
    <OneStationView stationId="R20"/>
  </div>,
  document.getElementById('tt_app'));
