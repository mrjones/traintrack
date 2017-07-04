import * as React from "react";
import * as ReactRouter from "react-router-dom";

import * as proto from './webclient_api_pb';

import { ApiDebugger, ApiRequestInfo } from './debug';
import { DataFetcher, DebuggableResult } from './datafetcher';

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
      apiInfo: new ApiRequestInfo("", null),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchLineList()
      .then((res: DebuggableResult<proto.LineList>) => {
        this.setState({lineList: res.data, apiInfo: new ApiRequestInfo(res.apiUrl, res.debugInfo)});
      });
  }

  public render(): JSX.Element {
    let lineLis = this.state.lineList.line.map((line: proto.Line) => {
      let c = "#" + line.colorHex;
      let liStyle = {
        background: c,
      };

      let className = line.active ? "active" : "inactive";
      return <li key={line.name} style={liStyle} className={className}>
        <ReactRouter.Link to={`/app/line/${line.name}`}>
          {line.name}
        </ReactRouter.Link>
      </li>;
    });

    return (<div>
            <ul className="lineList">{lineLis}</ul>
            <ApiDebugger requestInfo={this.state.apiInfo} />
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
