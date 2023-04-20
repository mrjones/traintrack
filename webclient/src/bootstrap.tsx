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

import * as History from "history"
import * as React from "react";
import * as ReactDOM from "react-dom";
import * as ReactDOMClient from "react-dom/client";
import * as ReactGA from "react-ga";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as ReduxThunk from "redux-thunk";

//import { BrowserRouter } from "react-g-analytics";
//import { BrowserRouter } from "react-router-dom";
import { Helmet } from "react-helmet";

import { LineViewRouterWrapper } from './lineview';
import { LinePickerRouterWrapper } from './navigation';
import { StationPageWrapper } from './station-view';
import { AboutPage } from './about-page';
import { PrefetcherDebuggerPage } from './debug';
import { Prefetcher } from './prefetcher';
import { TrainItineraryWrapper } from './train-itinerary';
import { TransferPageWrapper } from './transfer-view';

import { DataFetcher } from './datafetcher';
import { TTContext, TTState, initialState, transition } from './state-machine';

let context = new TTContext(new DataFetcher(0));
let store: Redux.Store<TTState> = Redux.createStore(
  Redux.combineReducers({core: transition}),
  {core: initialState},
  Redux.applyMiddleware((ReduxThunk.default.withExtraArgument(context) as ReduxThunk.ThunkMiddleware<TTState>)));

let prefetcher = new Prefetcher(ENABLE_PREFETCHING, context, store);
// <ReactRouter.Route path='/app/debug/prefetcher' element={<PrefetcherDebuggerPage prefetcher={prefetcher} />)} />

ReactGA.initialize("G-RVVV1VSRRM", { debug: false });


const history = History.createBrowserHistory();

const container = document.getElementById('tt_app');
const root = ReactDOMClient.createRoot(container);

const App = () => {
  const location = ReactRouter.useLocation();

  React.useEffect(() => {
    ReactGA.pageview(location.pathname + location.search);
    console.log("REPORTING TO GA: " + location.pathname + location.search);
  }, [location.pathname]);

  return <ReactRouter.Routes>
      <ReactRouter.Route path='/app/lines' element={<LinePickerRouterWrapper/>}/>
      <ReactRouter.Route path='/app/line/:lineId' element={<LineViewRouterWrapper/>}/>
      <ReactRouter.Route path='/app/station/:initialStationId/:visibilitySpec' element={<StationPageWrapper/>} />
      <ReactRouter.Route path='/app/station/:initialStationId' element={<StationPageWrapper/>} />
      <ReactRouter.Route path='/app/train/:trainId' element={<TrainItineraryWrapper/>}/>
      <ReactRouter.Route path='/app/transfer/:rootSpec/:transferSpec' element={<TransferPageWrapper/>}/>
      <ReactRouter.Route path='/app/transfer' element={<TransferPageWrapper/>}/>
      <ReactRouter.Route path='/app/about' element={<AboutPage/>}/>
      <ReactRouter.Route path='/app' element={<StationPageWrapper/>}/>
      <ReactRouter.Route path='/' element={<StationPageWrapper/>}/>
    </ReactRouter.Routes>

}

root.render(
  <ReactRedux.Provider store={store}>
    <div>
      <Helmet>
        <meta name="viewport" content="initial-scale=1" />
      </Helmet>
      <ReactRouter.BrowserRouter>
        <div className="app">
          <div className="app_title">
            <ReactRouter.Link to={`/app`}>
              <span className="first">Train</span><span className="second">Track</span>
            </ReactRouter.Link>
          </div>
          <div className="app_content">
            <App />
          </div>
        </div>
      </ReactRouter.BrowserRouter>
    </div>
  </ReactRedux.Provider>);

if (ENABLE_PREFETCHING) {
  console.log("Prefetching enabled");
  //  prefetcher.prefetchRecentStations();
  store.subscribe(() => prefetcher.kick());
} else {
  console.log("Prefetching disabled");
}

const bootstrapLoadTimeMs = new Date().getTime();
if (htmlLoadTimeMs) {
  console.log("boostrapLoadTimeMs: " + bootstrapLoadTimeMs +
              " (+" + (bootstrapLoadTimeMs - htmlLoadTimeMs) + "ms)");
}
