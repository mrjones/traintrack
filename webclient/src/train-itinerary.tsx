import * as React from "react";
import * as ReactRouter from "react-router";
import * as moment from "moment";

import * as proto from './webclient_api_pb';

import { DataFetcher, DebuggableResult } from './datafetcher';
import { ApiRequestInfo, ApiDebugger } from './debug';

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
      const lis = this.state.data.data.arrival.map((arrival: proto.TrainItineraryArrival) => {
        let name = "Unknown";
        const time = moment.unix(arrival.timestamp as number);
        if (arrival.station && arrival.station.name) {
          name = arrival.station.name;
        }
        return <li key={"" + arrival.timestamp}>{name} @ {time.format("LT")} ({time.fromNow()})</li>;
      });
      body = <ul>{lis}</ul>;
    }


    return <div>
      <h1>Train Itinerary {this.props.trainId}</h1>
      {body}
      <ApiDebugger requestInfo={new ApiRequestInfo(this.state.data.apiUrl, this.state.data.debugInfo)} />
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
