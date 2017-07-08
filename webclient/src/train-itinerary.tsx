import * as React from "react";
import * as ReactRouter from "react-router-dom";
import * as moment from "moment";

import * as proto from './webclient_api_pb';

import { DataFetcher, DebuggableResult } from './datafetcher';
import { ApiDebugger } from './debug';

class TrainItineraryState {
  data: DebuggableResult<proto.ITrainItinerary>;
  loaded: boolean;
}
class TrainItineraryProps {
  dataFetcher: DataFetcher;
  trainId: string;
}

export class TrainItinerary extends React.Component<TrainItineraryProps, TrainItineraryState> {
  constructor(props: TrainItineraryProps) {
    super(props);

    this.state = {
      data: new DebuggableResult<proto.ITrainItinerary>(null, ""),
      loaded: false,
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchTrainItinerary(this.props.trainId).then(
      (res: DebuggableResult<proto.ITrainItinerary>) => {
        this.setState({data: res, loaded: true});
      });
  }

  render(): JSX.Element {
    let body = <div>Loading...</div>;
    if (this.state.loaded) {
      const rows = this.state.data.data.arrival.map((arrival: proto.TrainItineraryArrival) => {
        const time = moment.unix(arrival.timestamp as number);
        let stationElt = <span>Unknown station</span>;
        if (arrival.station && arrival.station.name && arrival.station.id) {
          stationElt = <ReactRouter.Link to={`/app/station/${arrival.station.id}`}>
            {arrival.station.name}
          </ReactRouter.Link>;
        }
        return <tr key={"" + arrival.timestamp}>
          <td className="station">{stationElt}</td>
          <td className="arrivalTime">{time.format("LT")} ({time.fromNow()})</td>
        </tr>;
      });
      body = <table className="trainItinerary">
        <tbody>
          {rows}
        </tbody>
      </table>;
    }

    return <div className="page">
      <div className="pageTitle">Train Itinerary {this.props.trainId}</div>
      {body}
      <ApiDebugger datasFetched={[this.state.data]} />
    </div>;
  }
}

export class TrainItineraryWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render(): JSX.Element {
    return <TrainItinerary dataFetcher={new DataFetcher()} trainId={this.props.match.params.trainId} />;
  }
}
