import * as React from "react";

import * as proto from './webclient_api_pb';

export class ApiRequestInfo {
  public apiUrl: string;
  public debugInfo?: proto.IDebugInfo;

  constructor(apiUrl: string, debugInfo?: proto.IDebugInfo) {
    this.apiUrl = apiUrl;
    this.debugInfo = debugInfo;
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
    if (this.props.requestInfo.debugInfo) {
      processingTimeMessage = "(Server time: " + this.props.requestInfo.debugInfo.processingTimeMs + " ms)";
    }

    return <div>
      API: <a href={jsonLink}>{jsonLink}</a> {processingTimeMessage}
    </div>;
  }
};
