import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import thunk from "redux-thunk";

import { returntypeof } from 'react-redux-typescript';

import { Helmet } from "react-helmet";

import { LineViewRouterWrapper } from './lineview';
import { LinePickerRouterWrapper } from './navigation';
import { StationPageWrapper } from './station-view';
import { TrainItineraryWrapper } from './train-itinerary';

class TTContext {
  public contextStuff: string;

  public constructor() {
    this.contextStuff = "[from constructor]";
  }
}

type TTState = {
  stationId: string;
  stationName: string;

  loading: boolean;
}

const initialState: TTState = {
  stationId: "028",
  stationName: "Not loaded yet.",
  loading: false,
};

export enum TTActionTypes {
  START_CHANGE_STATION = "START_CHANGE_STATION",
  FINISH_CHANGE_STATION = "FINISH_CHANGE_STATION",
};

function startChangeStation(newStationId: string): StartChangeStationAction {
  console.log("startChangeStation");
  return {
    type: TTActionTypes.START_CHANGE_STATION,
    payload: newStationId,
  };
}

function finishChangeStation(newStationInfo: string): FinishChangeStationAction {
  console.log("finishChangeStation");
  return {
    type: TTActionTypes.FINISH_CHANGE_STATION,
    payload: newStationInfo,
  };
}

function changeStation(newStationId: string) {
  console.log("changeStation");
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, extraApi: TTContext) => {
    dispatch(startChangeStation(newStationId));

    setTimeout(() => {
      console.log("running change station callback");
      dispatch(finishChangeStation("Union Street " + extraApi.contextStuff));
    }, 2000);
  };
}

interface TTAction<T, P> {
  type: T;
  payload: P;
}

type StartChangeStationAction = TTAction<TTActionTypes.START_CHANGE_STATION, string>;
type FinishChangeStationAction = TTAction<TTActionTypes.FINISH_CHANGE_STATION, string>;

type TTActions = StartChangeStationAction | FinishChangeStationAction;

function reducer<T, P>(state: TTState = initialState, action: TTActions): TTState {
  console.log("REDUCER state.stationId = " + state.stationId);
  let partialState: Partial<TTState> | undefined;
  switch (action.type) {
  case TTActionTypes.START_CHANGE_STATION: {
    console.log("START_CHANGE_STATION -> " + action.payload);
    partialState = {
      loading: true,
      stationId: action.payload,
      stationName: "::Loading::",
    };
    break;
  }
  case TTActionTypes.FINISH_CHANGE_STATION: {
    console.log("FINISH_CHANGE_STATION -> " + action.payload);
    partialState = {
      stationName: action.payload,
      loading: false,
    };
    break;
  }
  }

  return partialState != null ? { ...state, ...partialState } : state;
}

let context = new TTContext();
let store = Redux.createStore(reducer, initialState, Redux.applyMiddleware(thunk.withExtraArgument(context)));

const mapStateToProps = (state: TTState, ownProps: FooComponentExplicitProps): FooComponentStateProps => ({
  message: "The station ID is: " + state.stationId + ", and name is: " + state.stationName + ", and tag is: " + ownProps.tag,
  loading: state.loading,
});

const dispatchToProps = (dispatch: Redux.Dispatch<TTState>): FooComponentDispatchProps => ({
  onChangeStation: (newId: string) => dispatch(changeStation(newId)),
});

interface FooComponentExplicitProps {
  tag: string;
}

interface FooComponentStateProps {
  message: string;
  loading: boolean;
}

interface FooComponentDispatchProps {
  onChangeStation(newId: string): any;
}

class ReduxFooComponent extends React.Component<FooComponentStateProps & FooComponentDispatchProps & FooComponentExplicitProps, {}> {
  public render(): JSX.Element {
    return (<div>
            Foo component: {this.props.message}
            <div><a href="#" onClick={this.props.onChangeStation.bind(this, "12345")}>Change</a></div>
            <div>Loading: {this.props.loading ? "yes" : "no"}</div>
            <div>Tag: {this.props.tag}</div>
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
    <ConnectedComponent tag="myTag"/>
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
