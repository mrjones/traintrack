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

import * as moment from "moment";
import * as React from "react";
import * as ReactRouter from "react-router-dom";

import * as proto from './webclient_api_pb';

import { DebuggableResult } from './datafetcher';
import { Prefetcher } from './prefetcher';

export class ClientDebugInfo {
  public fetchStarted: moment.Moment;
  public fetchCompleted: moment.Moment;

  public constructor(
    fetchStarted: moment.Moment, fetchCompleted: moment.Moment) {
    this.fetchStarted = fetchStarted;
    this.fetchCompleted = fetchCompleted;
  }

  public waitTimeMs(): number {
    return this.fetchCompleted.valueOf() - this.fetchStarted.valueOf();
  }
}

class ApiDebuggerProps {
  public datasFetched: DebuggableResult<any>[];
}

class ApiDebuggerState { }

export class ApiDebugger extends React.Component<ApiDebuggerProps, ApiDebuggerState> {

  public render(): JSX.Element {
    let requestElts = this.props.datasFetched.map((d: DebuggableResult<any>) => {
      return <SingleDataRequestDebugger key={d.apiUrl} request={d}/>;
    });

    return <div className="apiDebugger"><ReactRouter.Link to={"/app/about"}>About / Feedback</ReactRouter.Link>{requestElts}</div>;
  }
};

class SingleDataRequestDebuggerProps {
  public request: DebuggableResult<any>;
}
class SingleDataRequestDebuggerState { }

class SingleDataRequestDebugger extends React.Component<SingleDataRequestDebuggerProps, SingleDataRequestDebuggerState> {
  public render(): JSX.Element {
    if (!this.props.request.apiUrl) {
      return <div className="apiRequest">Loading...</div>;
    }

    let jsonLink: string = this.props.request.apiUrl + "?format=json";

    let components = [];

    let clientInfo = null;
    let serverInfo = null;

    if (this.props.request.serverDebugInfo) {
      serverInfo = this.props.request.serverDebugInfo;
    }
    if (this.props.request.clientDebugInfo) {
      clientInfo = this.props.request.clientDebugInfo;
    }

    if (clientInfo) {
      components.push("fetched=" + clientInfo.fetchStarted.format("h:mm:ss"));
      components.push("client_total=" + clientInfo.waitTimeMs() + "ms");
    }

    if (serverInfo) {
      components.push("server=" + serverInfo.processingTimeMs + "ms");
      components.push("server_build=" + serverInfo.buildVersion);
    }

    if (BUILD_LABEL) {
      components.push("client_build=" + BUILD_LABEL);
    }

    let prefetcherLink;
    if (process.env.NODE_ENV === "development") {
      prefetcherLink = <div><ReactRouter.Link to={`/app/debug/prefetcher`}>Prefetcher Status</ReactRouter.Link></div>;

    }

    return <div className="apiRequest">
      API: <a href={jsonLink}>{jsonLink}</a> <span className="debugDetails">({components.join(" ")})</span>
      {prefetcherLink}
    </div>;
  }
}

class PrefetcherDebuggerPageProps {
  public prefetcher: Prefetcher;
}

export class PrefetcherDebuggerPage extends React.Component<PrefetcherDebuggerPageProps, any> {
  constructor(props: any) {
    super(props);
  }

  public render() {
    return <div><h2>Prefetcher (Debug)</h2>{this.props.prefetcher.statusPage()}</div>;
  }
}
