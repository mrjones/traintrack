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
}

const initialState: TTState = {
  foo: "initial-test",
};

type TTAction = { }

function reducer(state: TTState = initialState, action: TTAction): TTState {
  return state;
}

let store = Redux.createStore(reducer);

const mapStateToProps = (state: TTState) => ({
  message: "The message is: " + state.foo,
});

const dispatchToProps = { };

const propsType = returntypeof(mapStateToProps);
type Props = typeof propsType & typeof dispatchToProps;
type State = {}

class ReduxFooComponent extends React.Component<Props, State> {
  public render(): JSX.Element {
    return <div>Foo component: {this.props.message}</div>;
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
