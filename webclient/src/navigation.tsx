import * as React from "react";
import * as ReactRouter from "react-router-dom";

import * as proto from './webclient_api_pb';

import { DataFetcher } from './datafetcher';

class LinePickerProps {
  public dataFetcher: DataFetcher;
}

class LinePickerState {
  public lineList: proto.LineList;
}

export default class LinePicker extends React.Component<LinePickerProps, any> {
  constructor(props: LinePickerProps) {
    super(props);
    this.state = {
      lineList: new proto.LineList(),
    };
  }

  public componentDidMount() {
    this.props.dataFetcher.fetchLineList()
      .then((lineList: proto.LineList) => {
        this.setState({lineList: lineList});
      });
  }

  public render(): JSX.Element {
    let lineLis = this.state.lineList.line.map((line: proto.Line) => {
      let c = "#" + line.colorHex;
      let liStyle = {
        background: c,
      };
      return <li key={line.name} style={liStyle}>
        <ReactRouter.Link to={`/singlepage/line/${line.name}`}>
          {line.name}
        </ReactRouter.Link>
      </li>;
    });

    return (<div>
            <ul className="lineList">{lineLis}</ul>
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