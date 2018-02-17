// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import * as Immutable from 'immutable';
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";
import * as proto from './webclient_api_pb';

import * as utils from './utils';

import { TTState } from './state-machine';

enum IncludeExclude { INCLUDE, EXCLUDE };

export class DimensionState<T> {
  private values: Immutable.Set<T>;
  private kind: IncludeExclude;
  private formatT: (v: T) => string;

  constructor(values: Immutable.Set<T>, kind: IncludeExclude, formatT: ((v: T) => string)) {
    this.values = values;
    this.kind = kind;
    this.formatT = formatT;
  }

  public clone(): DimensionState<T> {
    return new DimensionState<T>(this.values, this.kind, this.formatT);
  }

  public toggle(v: T) {
    if (this.values.has(v)) {
      this.values = this.values.delete(v);
    } else {
      this.values = this.values.add(v);
    }
  }

  public includes(v: T): boolean {
    if (this.kind === IncludeExclude.INCLUDE) {
      return this.values.contains(v);
    } else {
      return !this.values.contains(v);
    }
  }

  public toSpec(): string {
    let ret = "";
    ret += ((this.kind === IncludeExclude.INCLUDE) ? '+' : '-');
    this.values.forEach((v: T) => {
      ret += this.formatT(v);
    });

    return ret;
  }

  public static parseFromSpec<T>(
    spec: String,
    parseT: ((spec: string) => T),
    formatT: ((v: T) => string)): DimensionState<T> {

    let values = Immutable.Set<T>();
    let kind = IncludeExclude.EXCLUDE;
    if (spec.length >= 2 && (spec[0] === '+' || spec[0] === '-')) {
      if (spec[0] === '+') {
        kind = IncludeExclude.INCLUDE;
      }

      for (const specChar of spec.substring(1)) {
        values = values.add(parseT(specChar));
      }
    }

    return new DimensionState<T>(values, kind, formatT);
  }
}

export class VisibilityState {
  private directions: DimensionState<proto.Direction>;
  private lines: DimensionState<string>;
  private combined: boolean;

  constructor(directions: DimensionState<proto.Direction>,
              lines: DimensionState<string>,
              combined: boolean) {
    this.directions = directions;
    this.lines = lines;
    this.combined = combined;
  }

  public clone() {
    return new VisibilityState(
      this.directions.clone(), this.lines.clone(), this.combined);
  }

  public toggleCombined() {
    this.combined = !this.combined;
  }

  public isCombined() {
    return this.combined;
  }

  public toggleDirection(dir: proto.Direction) {
    this.directions.toggle(dir);
  }

  public includesDirection(dir: proto.Direction): boolean {
    return this.directions.includes(dir);
  }

  public toggleLine(line: string) {
    return this.lines.toggle(line);
  }

  public includesLine(line: string): boolean {
    return this.lines.includes(line);
  }

  public toSpec(): string {
    return this.directions.toSpec()
      + ":" + this.lines.toSpec()
      + (this.combined ? ":C" : "");
  }

  public static parseFromSpec(spec: string): VisibilityState {
    let specParts = spec.length > 0 ? spec.split(":") : [];

    return new VisibilityState(
      DimensionState.parseFromSpec<proto.Direction>(
        specParts.length > 0 ? specParts[0] : "",
        (dirSpec: String) => {
          if (dirSpec === "U") {
            return proto.Direction.UPTOWN;
          } else {
            // TODO(mrjones): error case
            return proto.Direction.DOWNTOWN;
          }
        },
        (dir: proto.Direction) => {
          if (dir === proto.Direction.UPTOWN) {
            return "U";
          } else {
            return "D";
          }
        }),
      DimensionState.parseFromSpec<string>(
        specParts.length > 1 ? specParts[1] : "",
        (lineSpec: string) => { return lineSpec; },
        (line: string) => { return line; }),
      (specParts.length > 2 && specParts[2] === "C"));
  }
}

class FilterControlDataProps {
  public allTrains: proto.StationStatus;
};
class FilterControlDispatchProps { };
class FilterControlExplicitProps {
  public stationId: string;
  public visibilityState: VisibilityState;
}
class FilterControlLocalState {
  public lineColors: Map<string, string>;  // TODO: move to props
  public expanded: boolean;
};

type FilterControlProps = FilterControlDataProps & FilterControlDispatchProps & FilterControlExplicitProps;

const mapStateToProps = (state: TTState, ownProps: FilterControlExplicitProps): FilterControlDataProps => {
  let maybeStation = state.core.stationDetails.get(ownProps.stationId);
  if (maybeStation !== undefined && maybeStation.valid) {
    return {
      allTrains: maybeStation.data.data,
    };
  } else {
    return {
      allTrains: new proto.StationStatus(),
    };
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): FilterControlDispatchProps => ({ });

export class FilterControl extends React.Component<FilterControlProps, FilterControlLocalState> {
  public constructor(props: any) {
    super(props);

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
      let visible = this.props.visibilityState.includesDirection(direction);
      let className = "toggleButton autowidth " + (visible ? "active" : "inactive");
      let name = utils.directionName(direction);

      let newVisibility = this.props.visibilityState.clone();
      newVisibility.toggleDirection(direction);
      let link = '/app/station/' + this.props.stationId + "/" + newVisibility.toSpec();
      togglers.push(<div key={utils.directionName(direction)} className={className}>
                      <ReactRouter.Link to={link}>{name}</ReactRouter.Link>
                    </div>);
    });

    togglers.push(<div key="sep1" className="toggleSeparator" />);

    utils.linesForStation(this.props.allTrains).map((line: string) => {
      let visible = this.props.visibilityState.includesLine(line);
      let style = {
        background: "#" + this.state.lineColors.get(line),
      };
      let className = "toggleButton " + (visible ? "active" : "inactive");
      let newVisibility = this.props.visibilityState.clone();
      newVisibility.toggleLine(line);
      let link = '/app/station/' + this.props.stationId + "/" + newVisibility.toSpec();
      togglers.push(<div key={line} className={className} style={style}>
                      <ReactRouter.Link to={link}>{line}</ReactRouter.Link>
                    </div>);
    });

    togglers.push(<div key="sep2" className="toggleSeparator" />);

    let className = "togglebutton autowidth " + (this.props.visibilityState.isCombined() ? "active" : "inactive");
    let newVisibility = this.props.visibilityState.clone();
    newVisibility.toggleCombined();
    let link = '/app/station/' + this.props.stationId + "/" + newVisibility.toSpec();
    togglers.push(<div key="mixing" className={className}><ReactRouter.Link to={link}>Combined</ReactRouter.Link></div>);

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
};

export let ConnectedFilterControl = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(FilterControl);
