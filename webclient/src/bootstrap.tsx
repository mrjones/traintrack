import * as React from "react";
import * as ReactDOM from "react-dom";
import * as moment from "moment";
import * as proto from './webclient_api_pb';

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
  stationName: string;
  data: proto.StationStatus;
};

class StationBoard extends React.Component<any, StationBoardState> {
  constructor(props: any) {
    super(props);
    this.state = {
      stationName: "Loading...",
      data: new proto.StationStatus(),
    };
  }

  public componentDidMount() {
    const xhr = new XMLHttpRequest();
    const component = this;
    xhr.onreadystatechange = function() {
      if (this.readyState === 4 && this.status === 200) {
        /*
        console.log("Deserializing: " + this.responseText);
        const a = proto.LineArrivals.deserializeBinary(this.responseText);
         */

        const bytes = new Uint8Array(xhr.response);
        //        const data = proto.StationStatus.deserializeBinary(bytes);
        const data = proto.StationStatus.decode(bytes);
        
        console.log("Fetched station: [" + data.name + "]");
        component.setState({
          stationName: data.name,
          data: data,
        });
      }
    };
    xhr.responseType = 'arraybuffer';
    xhr.open("GET", "http://localhost:3838/api/station/R20");
    xhr.send();    
  }
  
  public render() {
    var lineSet = this.state.data.line.map(
      (line: proto.LineArrivals) => { return <StationLine data={line} /> });
    
    return (<div>
  <h1>StationBoard</h1>
  Station name = {this.state.stationName}
  {lineSet}
    </div>);
  };
}

ReactDOM.render(
    <StationBoard stationName="test station"/>,
  document.getElementById('tt_app'));
