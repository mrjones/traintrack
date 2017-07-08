import * as React from "react";

import * as proto from './webclient_api_pb';

export class ClientDebugInfo {
  public cached: boolean;
  public clientWaitTimeMs: number;

  public constructor(cached: boolean, clientWaitTimeMs: number) {
    this.cached = cached;
    this.clientWaitTimeMs = clientWaitTimeMs;
  }
}

export class ApiRequestInfo {
  public apiUrl: string;
  public serverDebugInfo?: proto.IDebugInfo;
  public clientDebugInfo?: ClientDebugInfo;

  constructor(apiUrl: string, serverDebugInfo?: proto.IDebugInfo, clientDebugInfo?: ClientDebugInfo) {
    this.apiUrl = apiUrl;
    this.serverDebugInfo = serverDebugInfo;
    this.clientDebugInfo = clientDebugInfo;
  }
}

class ApiDebuggerProps {
  public requestInfo: ApiRequestInfo;
}

class ApiDebuggerState { }

export class ApiDebugger extends React.Component<ApiDebuggerProps, ApiDebuggerState> {

  public render(): JSX.Element {
    let jsonLink: string = this.props.requestInfo.apiUrl + "?format=textproto";

    let processingTimeMessage = "";
    if (this.props.requestInfo.serverDebugInfo) {
      processingTimeMessage = "server_only=" + this.props.requestInfo.serverDebugInfo.processingTimeMs + "ms";
    }

    let waitTimeMessage = "";
    if (this.props.requestInfo.clientDebugInfo) {
      waitTimeMessage = "client_total=" + this.props.requestInfo.clientDebugInfo.clientWaitTimeMs + "ms cached=" + this.props.requestInfo.clientDebugInfo.cached;
    }

    return <div className="apiDebugger">
      API: <a href={jsonLink}>{jsonLink}</a> ({processingTimeMessage} {waitTimeMessage})
    </div>;
  }
};
