// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import * as Immutable from "immutable";
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as moment from "moment";
import * as history from "history";
import querystring from "query-string";

import { webclient_api } from './webclient_api_pb';

import { Loadable, itemIsBeingLoaded } from './async';
import { DebuggableResult } from './datafetcher';
import { ApiDebugger } from './debug';
import { PubInfo } from './pub-info';
import { TTActionTypes, TTContext, TTState, InstallTrainItineraryAction, StartLoadingTrainItineraryAction } from './state-machine';
import { TTThunkDispatch } from './thunk-types';

export class TrainItineraryQueryParams {
  public static parseFrom(query: history.Search): TrainItineraryQueryParams {
    let parsed = querystring.parse(query, {arrayFormat: 'comma'});
    // It's obnoxious that
    // (a) this returns a "string | string[]" depending on whether the input
    //     has one or more matches.
    // (b) Immutable.Set treats "string" and "string[]" differently:
    //     "A JavaScript string is an iterable object, so if you create a
    //      Set of strings using Set("string"), you’ll actually get a Set
    //      of characters (['s', 't', 'r', 'i', 'n', 'g'])"
    //      - https://untangled.io/immutablejs-creating-sets/
    // So, we have to switch on the type of the object and handle them
    // differently.
    let highlights: string | string[] = parsed["highlight"];
    if (Array.isArray(highlights)) {
      return { highlightedStations: Immutable.Set<string>(highlights), };
    } else {
      return { highlightedStations: Immutable.Set.of<string>(highlights), };
    }
  }

  public highlightedStations: Immutable.Set<string>;
}

function installTrainItinerary(newTrainId: string, newTrainInfo: DebuggableResult<webclient_api.ITrainItinerary>): InstallTrainItineraryAction {
  return {
    type: TTActionTypes.INSTALL_TRAIN_ITINERARY,
    payload: [newTrainId, newTrainInfo],
  };
}

function startLoadingTrainItinerary(trainId: string): StartLoadingTrainItineraryAction {
  return {
    type: TTActionTypes.START_LOADING_TRAIN_ITINERARY,
    payload: trainId,
  };
}

function loadTrainItinerary(trainId: string) {
  return (dispatch: Redux.Dispatch, getState: () => TTState, context: TTContext) => {
    if (itemIsBeingLoaded(trainId, getState().core.trainItineraries)) {
      return;
    }
    dispatch(startLoadingTrainItinerary(trainId));
    // TODO(mrjones): check for errors which might wedge us in "loading"
    context.dataFetcher.fetchTrainItinerary(trainId).then(
      (result: DebuggableResult<webclient_api.ITrainItinerary>) => {
        dispatch(installTrainItinerary(trainId, result));
      });
  };
}

class TrainItineraryDataProps {
  public hasData: boolean;
  public loading: boolean;
  public data: DebuggableResult<webclient_api.ITrainItinerary>;
}
class TrainItineraryDispatchProps {
  public loadItinerary: (trainId: string) => any;
}
class TrainItineraryExplicitProps {
  public trainId: string;
  public queryParams: TrainItineraryQueryParams;
}
type TrainItineraryProps = TrainItineraryDataProps & TrainItineraryDispatchProps & TrainItineraryExplicitProps;

class TrainItineraryLocalState { }

export class TrainItinerary extends React.Component<TrainItineraryProps, TrainItineraryLocalState> {
  constructor(props: TrainItineraryProps) {
    super(props);

    this.state = { };
  }

  public componentDidMount() {
    this.props.loadItinerary(this.props.trainId);
  }

  public componentDidUpdate(nextProps: TrainItineraryProps) {
    if (this.props.trainId !== nextProps.trainId) {
      this.props.loadItinerary(nextProps.trainId);
    }
  }

  public render(): JSX.Element {
    let body = <div>Loading...</div>;
    let dataTs = moment.unix(0);
    if (this.props.hasData) {
      const rows = this.props.data.data.arrival.map((arrival: webclient_api.TrainItineraryArrival) => {
        const time = moment.unix(arrival.timestamp as number);
        let stationElt = <span>Unknown station</span>;
        if (arrival.station && arrival.station.name && arrival.station.id) {
          stationElt = <ReactRouter.Link to={`/app/station/${arrival.station.id}/::C?highlight=${this.props.trainId}`}>
            {arrival.station.name}
          </ReactRouter.Link>;
        } else if (arrival.station) {
          stationElt = <span>Unknown station ${arrival.station.id}</span>;
        }

        return <tr key={"" + arrival.timestamp} className={(arrival.station && arrival.station.id && this.props.queryParams.highlightedStations.has(arrival.station.id)) ? "highlighted" : ""}>
          <td className="station">{stationElt}</td>
          <td className="arrivalTime">{time.format("LT")} ({time.fromNow()})</td>
        </tr>;
      });
      body = <table className="trainItinerary">
        <tbody>
          {rows}
        </tbody>
      </table>;

      dataTs = moment.unix(this.props.data.data.dataTimestamp as number);
    }

    let lineStyle = {
      background: "#" + this.props.data.data.lineColorHex,
    };

    return <div className="page">
      <div className="pageTitle"><h2><span style={lineStyle} className="lineName">{this.props.data.data.line}</span> Train ({this.props.trainId})</h2></div>
      <PubInfo reloadFn={this.reloadData.bind(this)} pubTimestamp={dataTs} isLoading={this.props.loading}/>
      {body}
      <ApiDebugger datasFetched={[this.props.data]} />
    </div>;
  }

  private reloadData() {
    this.props.loadItinerary(this.props.trainId);
  }
}

const mapStateToProps = (state: TTState, ownProps: TrainItineraryExplicitProps): TrainItineraryDataProps => {
  let maybeData: Loadable<DebuggableResult<webclient_api.ITrainItinerary>> =
    state.core.trainItineraries.get(ownProps.trainId);
  if (maybeData !== undefined && maybeData.valid) {
    return {
      hasData: true,
      loading: maybeData.loading,
      data: maybeData.data,
    };
  } else {
    return {
      hasData: false,
      loading: true,
     data: new DebuggableResult<webclient_api.ITrainItinerary>(new webclient_api.TrainItinerary(), null, null),
    };
  }
};

const mapDispatchToProps = (dispatch: TTThunkDispatch): TrainItineraryDispatchProps => {
  return {
    loadItinerary: (trainId: string) => dispatch(loadTrainItinerary(trainId)),
  };
};

export let ConnectedTrainItinerary = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(TrainItinerary);

export const TrainItineraryWrapper = () => {
  const params = ReactRouter.useParams();
  const location = ReactRouter.useLocation();

  return <ConnectedTrainItinerary
  trainId={params.trainId} queryParams={TrainItineraryQueryParams.parseFrom(location.search)} />
}
