import * as React from "react";

import * as proto from './webclient_api_pb';

import { DebuggableResult } from './datafetcher';

export class ClientDebugInfo {
  public cached: boolean;
  public clientWaitTimeMs: number;

  public constructor(cached: boolean, clientWaitTimeMs: number) {
    this.cached = cached;
    this.clientWaitTimeMs = clientWaitTimeMs;
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
    let jsonLink: string = this.props.request.apiUrl + "?format=textproto";

    let processingTimeMessage = "";
    if (this.props.request.serverDebugInfo) {
      processingTimeMessage = "server_only=" + this.props.request.serverDebugInfo.processingTimeMs + "ms";
    }

    let waitTimeMessage = "";
    if (this.props.request.clientDebugInfo) {
      waitTimeMessage = "client_total=" + this.props.request.clientDebugInfo.clientWaitTimeMs + "ms cached=" + this.props.request.clientDebugInfo.cached + " server_build=" + this.props.request.serverDebugInfo.buildVersion;
    }

    return <div className="apiRequest">
      API: <a href={jsonLink}>{jsonLink}</a> ({processingTimeMessage} {waitTimeMessage})
    </div>;
  }
}
