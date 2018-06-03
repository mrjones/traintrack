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
import { AboutPage } from './about-page';
import { Prefetcher } from './prefetcher';
import { TrainItineraryWrapper } from './train-itinerary';
import { TransferPageWrapper } from './transfer-view';

import { DataFetcher } from './datafetcher';
import { TTContext, TTState, initialState, transition } from './state-machine';

let context = new TTContext(new DataFetcher(0));
let store: Redux.Store<TTState> = Redux.createStore(
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
              <ReactRouter.Route path='/app/station/:initialStationId/:visibilitySpec' component={StationPageWrapper} />
              <ReactRouter.Route path='/app/station/:initialStationId' component={StationPageWrapper} />
              <ReactRouter.Route path='/app/train/:trainId' component={TrainItineraryWrapper}/>
              <ReactRouter.Route path='/app/transfer/:rootSpec/:transferSpec' component={TransferPageWrapper}/>
              <ReactRouter.Route path='/app/transfer' component={TransferPageWrapper}/>
              <ReactRouter.Route path='/app/about' component={AboutPage}/>
              <ReactRouter.Route path='/app' component={StationPageWrapper}/>
              <ReactRouter.Route path='/' component={StationPageWrapper}/>
            </ReactRouter.Switch>
        </div>
      </div>
      </BrowserRouter>
    </div>
  </ReactRedux.Provider>,
  document.getElementById('tt_app'));

let prefetcher = new Prefetcher(context, store);
if (ENABLE_PREFETCHING) {
  console.log("Prefetching enabled");
  prefetcher.prefetchRecentStations();
} else {
  console.log("Prefetching disabled");
}
