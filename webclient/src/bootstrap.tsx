import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";

import { returntypeof } from 'react-redux-typescript';

import { Helmet } from "react-helmet";

import { LineViewRouterWrapper } from './lineview';
import { LinePickerRouterWrapper } from './navigation';
import { StationPageWrapper } from './station-view';
import { TrainItineraryWrapper } from './train-itinerary';

type TTState = {
  foo: string;
  stationId: string;
}

const initialState: TTState = {
  foo: "initial-test",
  stationId: "028",
};

export enum TTActionTypes {
  UPDATE_STATION_ID = "UPDATE_STATION_ID",
};

function updateStationId(newStationId: string): UpdateStationIdAction {
  return {
    type: TTActionTypes.UPDATE_STATION_ID,
    payload: newStationId,
  };
}

interface TTAction<T, P> {
  readonly type: T;
  readonly payload: P;
}

type UpdateStationIdAction = TTAction<TTActionTypes.UPDATE_STATION_ID, string>;

type TTActions = UpdateStationIdAction;

function reducer<T, P>(state: TTState = initialState, action: TTActions): TTState {
  console.log("REDUCER");
  let partialState: Partial<TTState> | undefined;
  switch (action.type) {
  case TTActionTypes.UPDATE_STATION_ID:
    console.log("UPDATE STATION ID");
    partialState = { stationId: action.payload };
  }
  return partialState != null ? { ...state, ...partialState } : state;
}

let store = Redux.createStore(reducer);

const mapStateToProps = (state: TTState): FooComponentStateProps => ({
  message: "The station is: " + state.stationId,
});

const dispatchToProps = (dispatch: Redux.Dispatch<TTState>): FooComponentDispatchProps => ({
  onChangeStation: (newId: string) => dispatch(updateStationId(newId)),
});

interface FooComponentStateProps {
  message: string;
}

interface FooComponentDispatchProps {
  onChangeStation(newId: string): any;
}

class ReduxFooComponent extends React.Component<FooComponentStateProps & FooComponentDispatchProps, {}> {
  public render(): JSX.Element {
    return (<div>
            Foo component: {this.props.message}
            <div><a href="#" onClick={this.props.onChangeStation.bind(this, "12345")}>Change</a></div>
            </div>);
  }
};

let ConnectedComponent = ReactRedux.connect(mapStateToProps, dispatchToProps)(ReduxFooComponent);

ReactDOM.render(
  <ReactRedux.Provider store={store}>
    <div>
      <Helmet>
        <meta name="viewport" content="initial-scale=1" />
      </Helmet>
    <ConnectedComponent />
    <ReactRouter.BrowserRouter>
        <div className="app">
          <div className="app_title">
            <ReactRouter.Link to={`/app`}>
              <span className="first">Train</span><span className="second">Track</span>
            </ReactRouter.Link>
          </div>
          <div className="app_content">
            <ReactRouter.Switch>
              <ReactRouter.Route path='/app/lines' component={LinePickerRouterWrapper}/>
              <ReactRouter.Route path='/app/line/:lineId' component={LineViewRouterWrapper}/>
              <ReactRouter.Route path='/app/station/:initialStationId' component={StationPageWrapper} />
              <ReactRouter.Route path='/app/train/:trainId' component={TrainItineraryWrapper}/>
              <ReactRouter.Route path='/app' component={StationPageWrapper}/>
              <ReactRouter.Route path='/' component={StationPageWrapper}/>
            </ReactRouter.Switch>
        </div>
      </div>
      </ReactRouter.BrowserRouter>
    </div>
  </ReactRedux.Provider>,
  document.getElementById('tt_app'));
