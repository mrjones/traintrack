import * as React from "react";
import * as ReactDOM from "react-dom";

import { Helmet } from "react-helmet";

import { LineViewRouterWrapper } from './lineview';
import { LinePickerRouterWrapper } from './navigation';
import { OneStationViewWrapper } from './station-view';
import { TrainItineraryWrapper } from './train-itinerary';

ReactDOM.render(
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
          <ReactRouter.Switch>
            <ReactRouter.Route path='/app/lines' component={LinePickerRouterWrapper}/>
            <ReactRouter.Route path='/app/line/:lineId' component={LineViewRouterWrapper}/>
            <ReactRouter.Route path='/app/station/:initialStationId' component={OneStationViewWrapper} />
            <ReactRouter.Route path='/app/train/:trainId' component={TrainItineraryWrapper}/>
            <ReactRouter.Route path='/app' component={OneStationViewWrapper}/>
            <ReactRouter.Route path='/' component={OneStationViewWrapper}/>
          </ReactRouter.Switch>
      </div>
    </div>
    </ReactRouter.BrowserRouter>
  </div>,
  document.getElementById('tt_app'));
