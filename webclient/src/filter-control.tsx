import * as React from "react";
import * as proto from './webclient_api_pb';

import * as utils from './utils';

class FilterControlProps {
  public updateFilterPredicateFn: (newFn: (l: proto.LineArrivals) => boolean) => void;
  public updateMixingFn: (mixMultipleLines: boolean) => void;
  public allTrains: proto.StationStatus;
};

class FilterControlState {
  public directionStates: Map<proto.Direction, boolean>;
  public lineStates: Map<string, boolean>;
  public lineColors: Map<string, string>;
  public mixMultipleLines: boolean;
};

export class FilterControl extends React.Component<FilterControlProps, FilterControlState> {
  public constructor(props: FilterControlProps) {
    super(props);

    this.state = {
      directionStates: new Map<proto.Direction, boolean>(),
      lineStates: new Map<string, boolean>(),
      lineColors: new Map<string, string>(),
      mixMultipleLines: false,
    };
  }

  public render(): JSX.Element {
    let togglers = new Array<JSX.Element>();
    this.state.directionStates.forEach((visible: boolean, direction: proto.Direction) => {
      let className = "toggleButton " + (visible ? "active" : "inactive");
      let name = direction.toString();
      if (direction === proto.Direction.UPTOWN) { name = '\u25b2'; }
      if (direction === proto.Direction.DOWNTOWN) { name = '\u25bc'; }
      togglers.push(<div key={utils.directionName(direction)} className={className}><a href="#" onClick={this.toggleDirection.bind(this, direction)}>{name}</a></div>);
    });

    this.state.lineStates.forEach((visible: boolean, line: string) => {
      let style = {
        background: "#" + this.state.lineColors.get(line),
      };
      let className = "toggleButton " + (visible ? "active" : "inactive");
      togglers.push(<div key={line} className={className} style={style}><a href="#" onClick={this.toggleLine.bind(this, line)}>{line}</a></div>);
    });

    let mixingWord = this.state.mixMultipleLines ? "Separate lines" : "Mix lines";
    togglers.push(<span key="mixing">[<a href="#" onClick={this.toggleMixing.bind(this)}>{mixingWord}</a>]</span>);

    return <div className="filterControl">
      {togglers}
      </div>;
  }

  public componentDidMount() {
    this.props.updateFilterPredicateFn(this.filterPredicate.bind(this));
  }

  public componentWillReceiveProps(nextProps: FilterControlProps) {
    let directionStates = new Map<proto.Direction, boolean>();
    let lineStates = new Map<string, boolean>();
    let lineColors = new Map<string, string>();
    nextProps.allTrains.line.map((line: proto.LineArrivals) => {
      const existingDState = this.state.directionStates.get(line.direction);
      if (existingDState === undefined) {
        directionStates.set(line.direction, true);
      } else {
        directionStates.set(line.direction, existingDState);
      }

      lineColors.set(line.line, line.lineColorHex);

      const existingLState = this.state.lineStates.get(line.line);
      if (existingLState === undefined) {
        lineStates.set(line.line, true);
      } else {
        lineStates.set(line.line, existingLState);
      }
    });

    this.setState({
      directionStates: directionStates,
      lineStates: lineStates,
      lineColors: lineColors,
    });
  }

  private filterPredicate(line: proto.LineArrivals): boolean {
    return this.directionVisible(line.direction) && this.lineVisible(line.line);
  }

  private directionVisible(direction: proto.Direction): boolean {
    const dState = this.state.directionStates.get(direction);
    if (dState === undefined) {
      return true;
    } else {
      return dState;
    }
  }

  private lineVisible(line: string): boolean {
    const dState = this.state.lineStates.get(line);
    if (dState === undefined) {
      return true;
    } else {
      return dState;
    }
  }

  private toggleDirection(direction: proto.Direction) {
    const dState = this.state.directionStates.get(direction);
    if (dState === undefined) {
      console.log("toggleDirection: Unknown direction: " + utils.directionName(direction));
      this.state.directionStates.set(direction, false);
    } else {
      this.state.directionStates.set(direction, !dState);
    }
  };

  private toggleLine(line: string) {
    const lState = this.state.lineStates.get(line);
    if (lState === undefined) {
      console.log("toggleLine: Unknown line: " + line);
      this.state.lineStates.set(line, false);
    } else {
      this.state.lineStates.set(line, !lState);
    }
  };

  private toggleMixing() {
    const newMixing = !this.state.mixMultipleLines;
    this.setState({mixMultipleLines: newMixing});
    this.props.updateMixingFn(newMixing);
  }
};
