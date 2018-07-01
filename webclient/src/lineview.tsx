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

import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as ReduxThunk from "redux-thunk";


import * as proto from './webclient_api_pb';

import { TTAsyncThunkAction, TTThunkDispatch } from './thunk-types';
import { Loadable, itemIsBeingLoaded } from './async';
import { ApiDebugger } from './debug';
import { DebuggableResult } from './datafetcher';
import { TTActionTypes, TTContext, TTState, InstallLineDetailsAction } from './state-machine';

function installLineDetails(lineId: string, stations: DebuggableResult<proto.StationList>): InstallLineDetailsAction {
  return {
    type: TTActionTypes.INSTALL_LINE_DETAILS,
    payload: [lineId, stations],
  };
}

function fetchLineDetails(lineId: string): TTAsyncThunkAction {
  return (dispatch: Redux.Dispatch, getState: () => TTState, context: TTContext): Promise<Redux.Action> => {
    if (itemIsBeingLoaded(lineId, getState().core.lineDetails)) {
      return;
    }
    // dispatch(startLoadingStationDetails(stationId));
    // TODO(mrjone): check for errors which might wedge us in "loading"
    context.dataFetcher.fetchStationsForLine(lineId)
      .then((stationList: DebuggableResult<proto.StationList>) => {
        dispatch(installLineDetails(lineId, stationList));
      });
  };
}

class LineViewDataProps {
  public stationList: DebuggableResult<proto.StationList>;
  public hasData: boolean;
}
class LineViewDispatchProps {
  public fetchLineData: (lineId: string) => any;
}
class LineViewExplicitProps {
  public lineId: string;
}
type LineViewProps = LineViewDataProps & LineViewDispatchProps & LineViewExplicitProps;
class LineViewLocalState { }

export default class LineView extends React.Component<LineViewProps, LineViewLocalState> {
  constructor(props: LineViewProps) {
    super(props);
    this.state = { };
  }

  private fetchDataIfNecessary(props: LineViewProps) {
    if (!props.hasData) {
      props.fetchLineData(props.lineId);
    }
  }

  public componentWillMount() {
    this.fetchDataIfNecessary(this.props);
  }

  public componentWillReceiveProps(nextProps: LineViewProps) {
    this.fetchDataIfNecessary(nextProps);
  }

  public render(): JSX.Element {
    if (!this.props.hasData) {
      return <div>Loading...</div>;
    }
    let stationLis = this.props.stationList.data.station.map((station: proto.Station) => {
      return <li key={station.name}>
        <ReactRouter.Link to={`/app/station/${station.id}`}>
          {station.name}
        </ReactRouter.Link>
      </li>;
    });

    return (<div>
              <h1>{this.props.lineId} Line</h1>
            <ul className="lineView">{stationLis}</ul>
            <ApiDebugger datasFetched={[this.props.stationList]}/>
            </div>);
  }
}

const mapStateToProps = (state: TTState, ownProps: LineViewExplicitProps): LineViewDataProps => {
  let maybeData: Loadable<DebuggableResult<proto.StationList>> =
    state.core.lineDetails.get(ownProps.lineId);
  if (maybeData !== undefined && maybeData.valid) {
    return {
      stationList: maybeData.data,
      hasData: true,
    };
  } else {
    return {
      stationList: new DebuggableResult<proto.StationList>(new proto.StationList(), null, null),
      hasData: false,
    };
  }
};

const mapDispatchToProps = (dispatch: TTThunkDispatch): LineViewDispatchProps => ({
  fetchLineData: (lineId: string) => dispatch(fetchLineDetails(lineId)),
});

export let ConnectedLineView = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(LineView);

export class LineViewRouterWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render(): JSX.Element {
    return <ConnectedLineView lineId={this.props.match.params.lineId} />;
  }
}
