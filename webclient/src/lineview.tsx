import * as React from "react";
import * as ReactRouter from "react-router-dom";

import * as proto from './webclient_api_pb';

import { DataFetcher } from './datafetcher';

class LineViewProps {
  public dataFetcher: DataFetcher;
  public lineId: string;
}

class LineViewState {
  public stationList: proto.StationList;
}

export default class LineView extends React.Component<LineViewProps, any> {
  constructor(props: LineViewProps) {
    super(props);
    this.state = {
      stationList: new proto.StationList(),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchStationsForLine(this.props.lineId)
      .then((stationList: proto.StationList) => {
        this.setState({stationList: stationList});
      });
  }

  public render(): JSX.Element {
    let stationLis = this.state.stationList.station.map((station: proto.Station) => {
      return <li key={station.name}>
        <ReactRouter.Link to={`/app/station/${station.id}`}>
          {station.name}
        </ReactRouter.Link>
      </li>;
    });

    return (<div>
              <h1>LineView: {this.props.lineId}</h1>
              <ul className="lineView">{stationLis}</ul>
            </div>);
  }
}

export class LineViewRouterWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render(): JSX.Element {
    return <LineView lineId={this.props.match.params.lineId} dataFetcher={new DataFetcher()}/>;
  }
}
