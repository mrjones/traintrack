import * as React from "react"
import * as proto from './webclient_api_pb';

class SubwayStatusProps {
  public status: proto.ISubwayStatusMessage[];
}

class SubwayStatusState {
  public expanded: boolean;
};

export class SubwayStatus extends React.Component<SubwayStatusProps, SubwayStatusState> {
  constructor(props: SubwayStatusProps) {
    super(props);

    this.state = {
      expanded: false,
    };
  }

  public render(): JSX.Element {
    if (!this.props.status || this.props.status.length == 0) {
      return null;
    }

    let toggleText = "Service status: " + this.props.status.length + " messages";

    if (!this.state.expanded) {
      return <div className="serviceStatus"><a href="#" onClick={this.toggleExpanded.bind(this)}>{toggleText}</a></div>;
    }

    let lis = this.props.status.map((msg: proto.ISubwayStatusMessage) => {
      let lines = msg.affectedLine.map((line: proto.AffectedLineStatus) => {
        return line.line + (line.direction == proto.Direction.UPTOWN ? "\u2191" : "\u2193");
      }).join(" ");
      return <li><strong>{msg.reasonName}: {lines}</strong><br/>{msg.summary}  (priority={msg.priority})</li>;
    });

    return <div className="serviceStatus"><a href="#" onClick={this.toggleExpanded.bind(this)}>{toggleText}</a><br/>[<strong>NOTE</strong> service status is still in development.]<ul>{lis}</ul></div>
  }

  private toggleExpanded() {
    this.setState({expanded: !this.state.expanded});
  }
}
