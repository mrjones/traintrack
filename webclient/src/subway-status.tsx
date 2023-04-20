import * as Immutable from 'immutable';
import * as moment from "moment";
import { webclient_api } from './webclient_api_pb';
import * as React from "react"

let UP_ARROW = "\u2191";
let DOWN_ARROW = "\u2193";

class SubwayStatusProps {
  public status: webclient_api.ISubwayStatusMessage[];
  public priorityLines: Immutable.Set<string>;
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

  private summarizeLines(lines: webclient_api.IAffectedLineStatus[][]): string {
    return this.directionMapToString(this.extractDirectionMap(lines));
  }

  private extractDirectionMap(lines: webclient_api.IAffectedLineStatus[][]): Map<string, Directions> {
    let lineBits: Map<string, Directions> = new Map();

    for (let lineSet of lines) {
      for (let line of lineSet) {
        if (this.props.priorityLines.isEmpty() ||
            this.props.priorityLines.contains(line.line)) {
          let existing = lineBits.get(line.line);
          if (!existing) {
            existing = new Directions();
          }

          if (line.direction == webclient_api.Direction.UPTOWN) {
            existing.uptown = true;
          } else if(line.direction == webclient_api.Direction.DOWNTOWN) {
            existing.downtown = true;
          }

          lineBits.set(line.line, existing);
        }
      }
    }

    return lineBits;
  }

  private directionMapToString(directionMap: Map<string, Directions>): string {
    return  Array.from(directionMap, ([line, directions]) => {
      return line + (directions.uptown ? UP_ARROW : "") + (directions.downtown ? DOWN_ARROW : "");
    }).join(" ");
  }

  public render(): JSX.Element {
    if (!this.props.status || this.props.status.length == 0) {
      return null;
    }

    let statusCopy = this.props.status.filter((msg: webclient_api.ISubwayStatusMessage) => {
      return !msg.affectedLine || msg.affectedLine.map((line: webclient_api.IAffectedLineStatus) => {
        return this.props.priorityLines.isEmpty() || this.props.priorityLines.contains(line.line);
      }).reduce((acc: boolean, next: boolean) => acc || next);
    });

    statusCopy.sort((a: webclient_api.ISubwayStatusMessage, b: webclient_api.ISubwayStatusMessage) => {
      if (a.priority != b.priority) {
        return (b.priority as number) - (a.priority as number);
      }

      return (b.publishTimestamp as number) - (a.publishTimestamp as number);
    });

    let overallSummary = this.summarizeLines(this.props.status.map((fullMessage: webclient_api.ISubwayStatusMessage) => {
      return fullMessage.affectedLine;
    }));
    let toggleText = "Affected Service: " + overallSummary;

    if (!this.state.expanded) {
      return <div className="serviceStatus"><a href="#" onClick={this.toggleExpanded.bind(this)}>{toggleText}</a></div>;
    }

    let lis = statusCopy.map((msg: webclient_api.ISubwayStatusMessage) => {
      let lines = this.summarizeLines([msg.affectedLine]);

      let publishText = "";
      if (msg.publishTimestamp && Number(msg.publishTimestamp) > 0) {
        const publishMoment = moment.unix(msg.publishTimestamp as number);

        publishText = publishMoment.fromNow();
      }

      let bodyText = this.state.longSituations.contains(msg.id) ? msg.longDescription : msg.summary;
      let toggleText = this.state.longSituations.contains(msg.id) ? "Less" : "More";
      let toggleLink = msg.longDescription ? <span>[<a href="#" onClick={this.toggleSituation.bind(this, msg.id)}>{toggleText}</a>]</span> : "";

      return <li key={msg.id}><strong>{msg.reasonName}: {lines}</strong> <span className="publishTimestamp">{publishText}</span><br/>{bodyText} {toggleLink}</li>;
    });

    return <div className="serviceStatus"><a href="#" onClick={this.toggleExpanded.bind(this)}>{toggleText}</a><ul>{lis}</ul></div>
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
