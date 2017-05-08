import * as React from "react";
import * as ReactDOM from "react-dom";
import './webclient_api_pb';

interface IStationBoardProps {
  stationName: string;
};

class StationBoard extends React.Component<IStationBoardProps, undefined> {
  public render() {
    const xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
      if (this.readyState === 4 && this.status === 200) {
        console.log("Deserializing: " + this.responseText);
        const a = proto.LineArrivals.deserializeBinary(this.responseText);
        a.setLine("foo");
        console.log(a.toString());
      }
    };
    xhr.open("GET", "http://linode.mrjon.es:3838/api/station/R20");
    xhr.send();

    return (<div>
  <h1>StationBoard</h1>
  Station name = {this.props.stationName}
    </div>);
  };
}

ReactDOM.render(
    <StationBoard stationName="test station"/>,
  document.getElementById('tt_app'));
