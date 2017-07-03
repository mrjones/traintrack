import * as React from "react";
import * as ReactRouter from "react-router-dom";

import * as proto from './webclient_api_pb';

import { DataFetcher, DebuggableResult } from './datafetcher';

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

class LinePickerProps {
  public dataFetcher: DataFetcher;
}

class LinePickerState {
  public lineList: proto.LineList;
  public apiUrl: String;
}

export default class LinePicker extends React.Component<LinePickerProps, any> {
  constructor(props: LinePickerProps) {
    super(props);
    this.state = {
      lineList: new proto.LineList(),
      apiUrl: "",
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchLineList()
      .then((res: DebuggableResult<proto.LineList>) => {
        this.setState({lineList: res.data, apiUrl: res.apiUrl});
      });
  }

  public render(): JSX.Element {
    let lineLis = this.state.lineList.line.map((line: proto.Line) => {
      let c = "#" + line.colorHex;
      let liStyle = {
        background: c,
      };
      return <li key={line.name} style={liStyle}>
        <ReactRouter.Link to={`/app/line/${line.name}`}>
          {line.name}
        </ReactRouter.Link>
      </li>;
    });

    return (<div>
            <ul className="lineList">{lineLis}</ul>
            <ApiDebugger apiUrl={this.state.apiUrl} />
            </div>);
  }
}

export class LinePickerRouterWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render(): JSX.Element {
    return <LinePicker dataFetcher={new DataFetcher()}/>;
  }
}
