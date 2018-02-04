import * as moment from "moment";
import * as React from "react";

import * as proto from './webclient_api_pb';

import { DebuggableResult } from './datafetcher';

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

    return <div className="apiDebugger">{requestElts}</div>;
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

    let jsonLink: string = this.props.request.apiUrl + "?format=textproto";

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

    return <div className="apiRequest">
      API: <a href={jsonLink}>{jsonLink}</a> <span className="debugDetails">({components.join(" ")})</span>
    </div>;
  }
}
