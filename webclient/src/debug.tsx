import * as React from "react";

class ApiDebuggerProps {
  public apiUrl: string;
}

class ApiDebuggerState { }

export class ApiDebugger extends React.Component<ApiDebuggerProps, ApiDebuggerState> {

  public render(): JSX.Element {
    let jsonLink: string = this.props.apiUrl + "?format=textproto";

    return <div>
      API: <a href={jsonLink}>{jsonLink}</a>
    </div>;
  }
};
