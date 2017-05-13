import * as React from "react";
import * as ReactDOM from "react-dom";
import * as moment from "moment";
import * as proto from './webclient_api_pb';

class StationPickerState {
  currentText: string;
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
      currentText: props.initialStationId,
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
      if (station.id == this.state.currentText) {
        console.log("Matched: " + station.name);
      }
    }
    this.props.stationPickedFn(this.state.currentText);
  }

  handleCurrentTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({currentText: e.currentTarget.value});
  }

  render() {
    return (<div>
  <form onSubmit={this.handleSubmit.bind(this)}>
    <input id="stationIdBox" type="text" value={this.state.currentText} onChange={this.handleCurrentTextChanged.bind(this)} />
    <input type="submit" />
  </form>
  <span>Station is: {this.state.currentText}</span>
    </div>);
  }

}

class OneStationViewProps {
  initialStationId: string;
}

class OneStationViewState {
  displayedStationId: string;
};

class OneStationView extends React.Component<OneStationViewProps, OneStationViewState> {
  constructor(props: OneStationViewProps) {
    super(props);

    this.state ={
      displayedStationId: props.initialStationId,
    };
  }

  handleStationChanged(newStationId: string) {
    this.setState({displayedStationId: newStationId});
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
  <div>
    <OneStationView initialStationId="R32"/>
  </div>,
  document.getElementById('tt_app'));
