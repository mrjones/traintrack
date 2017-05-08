import './webclient_api_pb';
import * as React from "react";
import * as ReactDOM from "react-dom";

interface IStationBoardProps {
  stationName: String;
};

class StationBoard extends React.Component<IStationBoardProps, undefined> {
  public render() {
    let xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
      if (this.readyState === 4 && this.status === 200) {
        console.log("Deserializing: " + this.responseText);
        let a = new proto.LineArrivals.deserializeBinary(this.responseText);
//        a.setLine("foo");
        console.log(a.toString());
      }
    };
    xhr.open("GET", "http://linode.mrjon.es:3838/api/station/R20");
    xhr.send();

    return <h1>StationBoard {this.props.stationName}</h1>;
  };
}

ReactDOM.render(
    <StationBoard stationName="test station"/>,
  document.getElementById('tt_app'));
