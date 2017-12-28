import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import thunk from "redux-thunk";


import { BrowserRouter } from "react-g-analytics";
import { Helmet } from "react-helmet";

import { LineViewRouterWrapper } from './lineview';
import { LinePickerRouterWrapper } from './navigation';
import { StationPageWrapper } from './station-view';
import { TrainItineraryWrapper } from './train-itinerary';
import { TransferPageWrapper } from './transfer-view';

import { DataFetcher } from './datafetcher';
import { TTContext, initialState, transition } from './state-machine';

let context = new TTContext(new DataFetcher());
let store = Redux.createStore(
  Redux.combineReducers({core: transition}),
  {core: initialState},
  Redux.applyMiddleware(thunk.withExtraArgument(context)));

ReactDOM.render(
  <ReactRedux.Provider store={store}>
    <div>
      <Helmet>
        <meta name="viewport" content="initial-scale=1" />
      </Helmet>
      <BrowserRouter id="UA-102249880-1">
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
              <ReactRouter.Route path='/app/transfer/:rootSpec/:transferSpec' component={TransferPageWrapper}/>
              <ReactRouter.Route path='/app/transfer' component={TransferPageWrapper}/>
              <ReactRouter.Route path='/app' component={StationPageWrapper}/>
              <ReactRouter.Route path='/' component={StationPageWrapper}/>
            </ReactRouter.Switch>
        </div>
      </div>
      </BrowserRouter>
    </div>
  </ReactRedux.Provider>,
  document.getElementById('tt_app'));
