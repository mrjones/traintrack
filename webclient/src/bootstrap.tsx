import * as React from "react";
import * as ReactDOM from "react-dom";

interface IStationBoardProps {
  stationName: String;
};

class StationBoard extends React.Component<IStationBoardProps, undefined> {
  public render() {
    return <h1>StationBoard {this.props.stationName}</h1>;
  };
}

ReactDOM.render(
    <StationBoard stationName="test station"/>,
  document.getElementById('tt_app'));
