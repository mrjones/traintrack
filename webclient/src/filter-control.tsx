import * as Immutable from 'immutable';
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as Redux from "redux";
import * as proto from './webclient_api_pb';
import * as History from "history";
import * as querystring from "query-string";

import * as utils from './utils';

import { TTActionTypes, TTState } from './state-machine';

export class FilterControlQueryParams {
  public static parseFrom(query: History.Search): FilterControlQueryParams {
    let parsed = querystring.parse(query);
    return {
      hiddenLines: parsed["hiddenLines"],
      hiddenDirections: parsed["hiddenDirections"],
      combined: parsed["combined"] === "true",
    };
  }

  public hiddenLines: String;
  public hiddenDirections: String;
  public combined: boolean;
}

class FilterControlDataProps {
  public allTrains: proto.StationStatus;
  public mixMultipleLines: boolean;
  public lineStates: Immutable.Map<string, boolean>;
  public directionStates: Immutable.Map<proto.Direction, boolean>;
};
class FilterControlDispatchProps {
  public onMixingChange: (newMixing: boolean) => any;
  public onLineVisibilityChange: (line: string, visible: boolean) => any;
  public onDirectionVisibilityChange: (line: proto.Direction, visible: boolean) => any;
};
class FilterControlExplicitProps {
  // TODO(mrjones): This is awkward, since it's only used in mapStateToProps, so it seems like it belongs elsewhere
  public stationId: string;
  public queryParams: FilterControlQueryParams;
}
class FilterControlLocalState {
  public lineColors: Map<string, string>;  // TODO: move to props
  public expanded: boolean;
};

type FilterControlProps = FilterControlDataProps & FilterControlDispatchProps & FilterControlExplicitProps;

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

const mapStateToProps = (state: TTState, ownProps: FilterControlExplicitProps): FilterControlDataProps => {
  let maybeStation = state.core.stationDetails.get(ownProps.stationId);
  if (maybeStation !== undefined && maybeStation.valid) {
    return {
      allTrains: maybeStation.data.data,
      mixMultipleLines: state.core.mixMultipleLines,
      lineStates: state.core.lineVisibility,
      directionStates: state.core.directionVisibility,
    };
  } else {
    return {
      allTrains: new proto.StationStatus(),
      mixMultipleLines: state.core.mixMultipleLines,
      lineStates: state.core.lineVisibility,
      directionStates: state.core.directionVisibility,
    };
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): FilterControlDispatchProps => ({
  onMixingChange: (newMixing: boolean) => dispatch(changeMixing(newMixing)),
  onLineVisibilityChange: (line: string, visible: boolean) => dispatch(changeLineVisibility(line, visible)),
  onDirectionVisibilityChange: (direction: proto.Direction, visible: boolean) => dispatch(changeDirectionVisibility(direction, visible)),
});

export class FilterControl extends React.Component<FilterControlProps, FilterControlLocalState> {
  public constructor(props: any) {
    super(props);

    if (this.props.queryParams.combined !== this.props.mixMultipleLines) {
      this.props.onMixingChange(this.props.queryParams.combined);
    }

    if (this.props.queryParams.hiddenLines) {
      for (const line of this.props.queryParams.hiddenLines) {
        // TODO(mrjones): re-show visible lines
        this.props.onLineVisibilityChange(line, false);
      }
    }

    if (this.props.queryParams.hiddenDirections) {
      for (const dirChar of this.props.queryParams.hiddenDirections) {
        if (dirChar === 'U') {
          this.props.onDirectionVisibilityChange(proto.Direction.UPTOWN, false);
        } else if (dirChar === 'D') {
          this.props.onDirectionVisibilityChange(proto.Direction.DOWNTOWN, false);
        } else {
          console.log("Malformed hiddenDirection string: " + this.props.queryParams.hiddenDirections);
        }
      }
    }

    this.state = {
      lineColors: new Map<string, string>(),
      expanded: false,
    };
  }

  public render(): JSX.Element {
    if (!this.state.expanded) {
      return <div className="filterControl">
        <a className="toggleExpander" href="#" onClick={this.toggleExpanded.bind(this)}>Filter +</a>
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
      <div key="filter" className="toggleButton autowidth"><a href="#" onClick={this.toggleExpanded.bind(this)}>Filter -</a></div>
      <div key="sep3" className="toggleSeparator" />
      {togglers}
      </div>;
  }

  public componentWillReceiveProps(nextProps: FilterControlProps) {
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
