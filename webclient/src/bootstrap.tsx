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

import { DataFetcher, DebuggableResult } from './datafetcher';
import { TTActionTypes, TTState, StartChangeStationAction, FinishChangeStationAction, InstallStationDetailsAction, initialState, transition } from './state-machine';

import * as proto from './webclient_api_pb';

class TTContext {
  public dataFetcher: DataFetcher;

  public constructor(dataFetcher: DataFetcher) {
    this.dataFetcher = dataFetcher;
  }
}

function startChangeStation(newStationId: string): StartChangeStationAction {
  console.log("startChangeStation");
  return {
    type: TTActionTypes.START_CHANGE_STATION,
    payload: newStationId,
  };
}

function finishChangeStation(): FinishChangeStationAction {
  console.log("finishChangeStation");
  return {
    type: TTActionTypes.FINISH_CHANGE_STATION,
    payload: {},
  };
}

function installStationDetails(newStationId: string, newStationInfo: DebuggableResult<proto.StationStatus>): InstallStationDetailsAction {
  return {
    type: TTActionTypes.INSTALL_STATION_DETAILS,
    payload: [newStationId, newStationInfo],
  };
}

function changeStation(newStationId: string) {
  console.log("changeStation");
  return (dispatch: Redux.Dispatch<TTState>, getState: () => TTState, context: TTContext) => {
    dispatch(startChangeStation(newStationId));

    let state: TTState = getState();
    if (state.stationDetails.has(newStationId)) {
      // &&         state.stationDetails.get(newStationId).data.dataTimestamp
      // TODO(mrjones): Split into multiple actions to avoid re-writing?
      dispatch(finishChangeStation());
    } else {
      console.log("Fetching " + newStationId);
      context.dataFetcher.fetchStationStatus(newStationId)
        .then((result: DebuggableResult<proto.StationStatus>) => {
          console.log("Got result: " + result.data.name);
          dispatch(installStationDetails(newStationId, result));
          dispatch(finishChangeStation());
        });
    }
  };
}

let context = new TTContext(new DataFetcher());
let store = Redux.createStore(transition, initialState, Redux.applyMiddleware(thunk.withExtraArgument(context)));

const mapStateToProps = (state: TTState, ownProps: FooComponentExplicitProps): FooComponentStateProps => {
  if (state.stationDetails.has(state.currentStationId)) {
    let details = state.stationDetails.get(state.currentStationId);
    return {
      message: "The station ID is: " + state.currentStationId + ", and name is: " + details.data.name + ", and tag is: " + ownProps.tag + ", and the data timestamp is" + details.data.dataTimestamp,
      loading: state.loading,
    };
  } else {
    return {
      message: "Data not loaded",
      loading: state.loading,
    };
  }
};

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

interface FooComponentState {
  currentText: string;
}

class ReduxFooComponent extends React.Component<FooComponentStateProps & FooComponentDispatchProps & FooComponentExplicitProps, FooComponentState> {

  public constructor() {
    super();
    this.state = {
      currentText: "602",
    };
  }

  public render(): JSX.Element {
    return (<div>
            Foo component: {this.props.message}
            <div>
              <input type="text" value={this.state.currentText} onChange={this.handleTextChanged.bind(this)} />
            <input type="submit" onClick={this.handleSubmit.bind(this)}/>
            </div>
            <div><a href="#" onClick={this.props.onChangeStation.bind(this, "602")}>Change</a></div>
            <div>Loading: {this.props.loading ? "yes" : "no"}</div>
            <div>Tag: {this.props.tag}</div>
            </div>);
  }

  private handleTextChanged(e: React.FormEvent<HTMLInputElement>) {
    this.setState({
      currentText: e.currentTarget.value,
    });
  }

  private handleSubmit() {
    this.props.onChangeStation(this.state.currentText);
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
