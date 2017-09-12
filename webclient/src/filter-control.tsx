import * as Immutable from 'immutable';
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as Redux from "redux";
import * as proto from './webclient_api_pb';

import * as utils from './utils';

import { TTActionTypes, TTState } from './state-machine';

class FilterControlProps {
  public updateFilterPredicateFn: (newFn: (l: proto.LineArrivals) => boolean) => void;
  public updateMixingFn: (mixMultipleLines: boolean) => void;
  public allTrains: proto.StationStatus;
};

class FilterControlState {
  public directionStates: Map<proto.Direction, boolean>;
  public lineStates: Map<string, boolean>;
  public lineColors: Map<string, string>;
  public expanded: boolean;
};

class FilterControlStateProps {
  public allTrains: proto.StationStatus;
  public mixMultipleLines: boolean;
  public lineStates: Immutable.Map<string, boolean>;
  public directionStates: Immutable.Map<proto.Direction, boolean>;
};
class FilterControlDispatchProps {
  public onMixingChange: (newMixing: boolean) => any;
  public onFilterPredicateChange: (newPredicate: (l: proto.LineArrivals) => boolean) => any;
  public onLineVisibilityChange: (line: string, visible: boolean) => any;
  public onDirectionVisibilityChange: (line: proto.Direction, visible: boolean) => any;
};
class FilterControlExplicitProps { }
class FilterControlLocalState {
  public lineColors: Map<string, string>;
  public mixMultipleLines: boolean;
  public expanded: boolean;
};

function changeLineVisibility(line: string, visible: boolean) {
  return {
    type: TTActionTypes.CHANGE_LINE_VISIBILITY,
    payload: [line, visible],
  };
}

function changeDirectionVisibility(direction: proto.Direction, visible: boolean) {
  return {
    type: TTActionTypes.CHANGE_DIRECTION_VISIBILITY,
    payload: [direction, visible],
  };
}

function changeMixing(newMixing: boolean) {
  return {
    type: TTActionTypes.CHANGE_LINE_MIXING,
    payload: newMixing,
  };
}

function changeFilterPredicate(newPredicate: (l: proto.LineArrivals) => boolean) {
  return {
    type: TTActionTypes.CHANGE_FILTER_PREDICATE,
    payload: newPredicate,
  };
}

const mapStateToProps = (state: TTState, ownProps: FilterControlExplicitProps): FilterControlStateProps => {
  if (state.stationDetails.has(state.currentStationId)) {
    return {
      allTrains: state.stationDetails.get(state.currentStationId).data,
      mixMultipleLines: state.mixMultipleLines,
      lineStates: state.lineVisibility,
      directionStates: state.directionVisibility,
    };
  } else {
    return {
      allTrains: new proto.StationStatus(),
      mixMultipleLines: state.mixMultipleLines,
      lineStates: state.lineVisibility,
      directionStates: state.directionVisibility,
    };
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): FilterControlDispatchProps => ({
  onMixingChange: (newMixing: boolean) => dispatch(changeMixing(newMixing)),
  onFilterPredicateChange: (newPredicate: (l: proto.LineArrivals) => boolean) => dispatch(changeFilterPredicate(newPredicate)),
  onLineVisibilityChange: (line: string, visible: boolean) => dispatch(changeLineVisibility(line, visible)),
  onDirectionVisibilityChange: (direction: proto.Direction, visible: boolean) => dispatch(changeDirectionVisibility(direction, visible)),
});

export class FilterControl extends React.Component<FilterControlStateProps & FilterControlDispatchProps & FilterControlExplicitProps, FilterControlLocalState> {
  public constructor(props: any) {
    super(props);

    this.state = {
      lineColors: new Map<string, string>(),
      mixMultipleLines: false,
      expanded: false,
    };
  }

  public render(): JSX.Element {
    if (!this.state.expanded) {
      return <div className="filterControl">
        <a className="toggleExpander" href="#" onClick={this.toggleExpanded.bind(this)}>Filter&nbsp;&raquo;</a>
        </div>;
    }
    let togglers = new Array<JSX.Element>();
    utils.directionsForStation(this.props.allTrains).map((direction: proto.Direction) => {
      let visible = this.props.directionStates.get(direction);
      if (visible === undefined) {
        visible = true;
      }
      let className = "toggleButton autowidth " + (visible ? "active" : "inactive");
      let name = utils.directionName(direction);
//      if (direction === proto.Direction.UPTOWN) { name = String.fromCodePoint(0x25B2); }
//      if (direction === proto.Direction.DOWNTOWN) { name = String.fromCodePoint(0x25BC); }
      togglers.push(<div key={utils.directionName(direction)} className={className}><a href="#" onClick={this.toggleDirection.bind(this, direction)}>{name}</a></div>);
    });

    togglers.push(<div key="sep1" className="toggleSeparator" />);

    utils.linesForStation(this.props.allTrains).map((line: string) => {
      let visible = this.props.lineStates.get(line);
      if (visible === undefined) {
        visible = true;
      }
      let style = {
        background: "#" + this.state.lineColors.get(line),
      };
      let className = "toggleButton " + (visible ? "active" : "inactive");
      togglers.push(<div key={line} className={className} style={style}><a href="#" onClick={this.toggleLine.bind(this, line)}>{line}</a></div>);
    });

    togglers.push(<div key="sep2" className="toggleSeparator" />);

    let className = "togglebutton autowidth " + (this.props.mixMultipleLines ? "active" : "inactive");
//    let mixingWord = this.state.mixMultipleLines ? "Separate" : "Group by line";
    togglers.push(<div key="mixing" className={className}><a href="#" onClick={this.toggleMixing.bind(this)}>Combined</a></div>);

    return <div className="filterControl">
      <div key="filter" className="toggleButton autowidth"><a href="#" onClick={this.toggleExpanded.bind(this)}>&laquo;&nbsp;Filter</a></div>
      <div key="sep3" className="toggleSeparator" />
      {togglers}
      </div>;
  }

  public componentDidMount() {
    this.props.onFilterPredicateChange(this.filterPredicate.bind(this));

    /*
    console.log("FilterControl :: componentDidMount");

    let directionStates = new Map<proto.Direction, boolean>();
    let lineStates = new Map<string, boolean>();
    let lineColors = new Map<string, string>();
    this.props.allTrains.line.map((line: proto.LineArrivals) => {
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
    */
  }

  public componentWillReceiveProps(nextProps: FilterControlStateProps & FilterControlDispatchProps & FilterControlExplicitProps) {
    let lineColors = new Map<string, string>();
    nextProps.allTrains.line.map((line: proto.LineArrivals) => {
      lineColors.set(line.line, line.lineColorHex);
    });

    this.setState({
      lineColors: lineColors,
    });
  }

  private toggleExpanded() {
    this.setState({expanded: !this.state.expanded});
  }

  private filterPredicate(line: proto.LineArrivals): boolean {
    return this.directionVisible(line.direction) && this.lineVisible(line.line);
  }

  private directionVisible(direction: proto.Direction): boolean {
    const dState = this.props.directionStates.get(direction);
    if (dState === undefined) {
      return true;
    } else {
      return dState;
    }
  }

  private lineVisible(line: string): boolean {
    const dState = this.props.lineStates.get(line);
    if (dState === undefined) {
      return true;
    } else {
      return dState;
    }
  }

  private toggleDirection(direction: proto.Direction) {
    let oldVisibility = this.props.directionStates.get(direction);
    if (oldVisibility === undefined) {
      oldVisibility = true;
    }
    this.props.onDirectionVisibilityChange(direction, !oldVisibility);
  };

  private toggleLine(line: string) {
    let oldVisibility = this.props.lineStates.get(line);
    if (oldVisibility === undefined) {
      oldVisibility = true;
    }

    this.props.onLineVisibilityChange(line, !oldVisibility);
  };

  private toggleMixing() {
    this.props.onMixingChange(!this.props.mixMultipleLines);
  }
};

export let ConnectedFilterControl = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(FilterControl);
