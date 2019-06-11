import * as Immutable from 'immutable';
import * as moment from "moment";
import * as proto from './webclient_api_pb';
import * as React from "react"

let UP_ARROW = "\u2191";
let DOWN_ARROW = "\u2193";

class SubwayStatusProps {
  public status: proto.ISubwayStatusMessage[];
}

class SubwayStatusState {
  public expanded: boolean;
  public longSituations: Immutable.Set<string>;
};

class Directions {
  public uptown: boolean;
  public downtown: boolean;
};

export class SubwayStatus extends React.Component<SubwayStatusProps, SubwayStatusState> {
  constructor(props: SubwayStatusProps) {
    super(props);

    this.state = {
      expanded: false,
      longSituations: Immutable.Set.of(),
    };
  }

  private summarizeLines(lines: proto.IAffectedLineStatus[][]): string {
    return this.directionMapToString(this.extractDirectionMap(lines));
  }

  private extractDirectionMap(lines: proto.IAffectedLineStatus[][]): Map<string, Directions> {
    let lineBits: Map<string, Directions> = new Map();

    for (let lineSet of lines) {
      for (let line of lineSet) {
        let existing = lineBits.get(line.line);
        if (!existing) {
          existing = new Directions();

        }

        if (line.direction == proto.Direction.UPTOWN) {
          existing.uptown = true;
        } else if(line.direction == proto.Direction.DOWNTOWN) {
          existing.downtown = true;
        }

        lineBits.set(line.line, existing);
      }
    }

    return lineBits;
  }

  private directionMapToString(directionMap: Map<string, Directions>): string {
    return  Array.from(directionMap, ([line, directions]) => {
      return line + " " + (directions.uptown ? UP_ARROW : "") + (directions.downtown ? DOWN_ARROW : "");
    }).join(" ");
  }

  public render(): JSX.Element {
    if (!this.props.status || this.props.status.length == 0) {
      return null;
    }

    // TODO(mrjones): Do this without a copy and re-sort every time.
    let statusCopy = [...this.props.status];
    statusCopy.sort((a: proto.ISubwayStatusMessage, b: proto.ISubwayStatusMessage) => {
      if (a.priority != b.priority) {
        return (b.priority as number) - (a.priority as number);
      }

      return (b.publishTimestamp as number) - (a.publishTimestamp as number);
    });

    let overallSummary = this.summarizeLines(this.props.status.map((fullMessage: proto.ISubwayStatusMessage) => {
      return fullMessage.affectedLine;
    }));
    let toggleText = "Service status: " + overallSummary;

    if (!this.state.expanded) {
      return <div className="serviceStatus"><a href="#" onClick={this.toggleExpanded.bind(this)}>{toggleText}</a></div>;
    }

    let lis = statusCopy.map((msg: proto.ISubwayStatusMessage) => {
      let lines = this.summarizeLines([msg.affectedLine]);

      let publishText = "";
      if (msg.publishTimestamp && msg.publishTimestamp > 0) {
        const publishMoment = moment.unix(msg.publishTimestamp as number);

        publishText = publishMoment.fromNow();
      }

      let bodyText = this.state.longSituations.contains(msg.id) ? msg.longDescription : msg.summary;
      let toggleText = this.state.longSituations.contains(msg.id) ? "Less" : "More";

      return <li><strong>{msg.reasonName}: {lines}</strong> <span className="publishTimestamp">{publishText}</span><br/>{bodyText}  (priority={msg.priority}) [<a href="#" onClick={this.toggleSituation.bind(this, msg.id)}>{toggleText}</a>]</li>;
    });

    return <div className="serviceStatus"><a href="#" onClick={this.toggleExpanded.bind(this)}>{toggleText}</a><br/>[<strong>NOTE</strong> service status is still in development.]<ul>{lis}</ul></div>
  }

  private toggleExpanded() {
    this.setState({expanded: !this.state.expanded});
  }

  private toggleSituation(id: string) {
    if (this.state.longSituations.includes(id)) {
      this.setState({longSituations: this.state.longSituations.remove(id)});
    } else {
      this.setState({longSituations: this.state.longSituations.add(id)});
    }
  }
}
