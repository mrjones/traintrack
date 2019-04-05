import * as React from "react"
import * as proto from './webclient_api_pb';

class SubwayStatusProps {
  public status: proto.ISubwayStatusMessage[];
}

export class SubwayStatus extends React.Component<SubwayStatusProps, any> {
  constructor(props: SubwayStatusProps) {
    super(props);
  }

  public render(): JSX.Element {
    let lis = this.props.status.map((msg: proto.ISubwayStatusMessage) => {
      let lines = msg.affectedLine.map((line: proto.AffectedLineStatus) => {
        return line.line + (line.direction == proto.Direction.UPTOWN ? "U" : "D");
      }).join(" ");
      return <li>{lines} {msg.summary}</li>;
    });

    return <div>Subway Status: <ul>{lis}</ul></div>
  }
}
