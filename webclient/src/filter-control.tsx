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
import { webclient_api } from './webclient_api_pb';

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

    const elemStrs = this.values.map((v: T) => { return this.formatT(v); });

    // Explicitly include a leading '.'.
    // There was an older style of spec which didn't use explicit dot
    // delimiters, and just used a character per element.  We still support
    // that format, in order to not break bookmarks.
    //
    // Including an explicit dot, even for single-elements lists, ensures
    // that we know we are using a new-style spec.  This avoids the ambiguity
    // of a spec like: "-:-GS:-" which could be interpreted as either:
    //    - Filter out "G" and "S" in the old character-based spec
    //    - Filter out "GS" (Grand Central Shuttle), in the new delimited spec
    // Instead we will represent the new spec as ".GS".
    ret += '.';
    ret += elemStrs.join('.');

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

        const elemsPart = spec.substring(1);

        if (elemsPart.startsWith('.')) {
          // New format: explicitly dot delimited
          // Manually discard leading dot:
          const elemsPartMinusDot = elemsPart.substring(1);

          if (elemsPartMinusDot.length > 0) {
            for (const specElem of elemsPartMinusDot.split('.')) {
              let parsed = parseT(specElem);
              values = values.add(parseT(specElem));
            }
          }
        } else {
          // Old format: one character per element
          for (const elemChar of elemsPart) {
            values = values.add(parseT(elemChar));
          }
        }
      }

      return new DimensionState<T>(values, kind, formatT);
    }
}

export class VisibilityState {
  private directions: DimensionState<webclient_api.Direction>;
  private lines: DimensionState<string>;
  private combined: boolean;

  // TODO(mrjones): Come up with a real experiment framework
  private showAlphaFeatures: boolean;

  constructor(directions: DimensionState<webclient_api.Direction>,
              lines: DimensionState<string>,
              combined: boolean,
              showAlphaFeatures: boolean) {
    this.directions = directions;
    this.lines = lines;
    this.combined = combined;
    this.showAlphaFeatures = showAlphaFeatures;
  }

  public clone() {
    return new VisibilityState(
      this.directions.clone(), this.lines.clone(), this.combined, this.showAlphaFeatures);
  }

  public toggleCombined() {
    this.combined = !this.combined;
  }

  public isCombined() {
    return this.combined;
  }

  public toggleDirection(dir: webclient_api.Direction) {
    this.directions.toggle(dir);
  }

  public includesDirection(dir: webclient_api.Direction): boolean {
    return this.directions.includes(dir);
  }

  public toggleLine(line: string) {
    return this.lines.toggle(line);
  }

  public includesLine(line: string): boolean {
    return this.lines.includes(line);
  }

  public shouldShowAlphaFeatures(): boolean {
    return this.showAlphaFeatures;
  }

  public toSpec(): string {
    return this.directions.toSpec()
      + ":" + this.lines.toSpec()
      + (this.combined ? ":C" : ":")
      + (this.showAlphaFeatures ? ":ALPHA" : ":");
  }

  public static parseFromSpec(spec: string): VisibilityState {
    let specParts = spec.length > 0 ? spec.split(":") : [];

    return new VisibilityState(
      DimensionState.parseFromSpec<webclient_api.Direction>(
        specParts.length > 0 ? specParts[0] : "",
        (dirSpec: String) => {
          if (dirSpec === "U") {
            return webclient_api.Direction.UPTOWN;
          } else {
            // TODO(mrjones): error case
            return webclient_api.Direction.DOWNTOWN;
          }
        },
        (dir: webclient_api.Direction) => {
          if (dir === webclient_api.Direction.UPTOWN) {
            return "U";
          } else {
            return "D";
          }
        }),
      DimensionState.parseFromSpec<string>(
        specParts.length > 1 ? specParts[1] : "",
        (lineSpec: string) => { return lineSpec; },
        (line: string) => { return line; }),
      (specParts.length > 2 && specParts[2] === "C"),
      (specParts.length > 3 && specParts[3] === "ALPHA"));
  }
}

class FilterControlDataProps {
  public allTrains: webclient_api.StationStatus;
};
class FilterControlDispatchProps { };
class FilterControlExplicitProps {
  public stationId: string;
  public visibilityState: VisibilityState;
  public queryParamsToPropagate: Immutable.Set<String>;
}
class FilterControlLocalState {
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
      allTrains: new webclient_api.StationStatus(),
    };
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch): FilterControlDispatchProps => ({ });

export class FilterControl extends React.Component<FilterControlProps, FilterControlLocalState> {
  public constructor(props: any) {
    super(props);

    this.state = {
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
    let queryString = this.props.queryParamsToPropagate.isEmpty() ? "" : "?" + this.props.queryParamsToPropagate.join("&");

    utils.directionsForStation(this.props.allTrains).map((direction: webclient_api.Direction) => {
      let visible = this.props.visibilityState.includesDirection(direction);
      let className = "toggleButton autowidth " + (visible ? "active" : "inactive");
      let name = utils.directionName(null, direction);

      let newVisibility = this.props.visibilityState.clone();
      newVisibility.toggleDirection(direction);
      let link = '/app/station/' + this.props.stationId + "/" + newVisibility.toSpec() + queryString;
      togglers.push(<div key={utils.directionName(null, direction)} className={className}>
                      <ReactRouter.Link to={link}>{name}</ReactRouter.Link>
                    </div>);
    });

    togglers.push(<div key="sep1" className="toggleSeparator" />);

    let lineColorsByName = new Map<string, string>();
    this.props.allTrains.line.map((line: webclient_api.LineArrivals) => {
      lineColorsByName.set(line.line, line.lineColorHex);
    });

    utils.linesForStation(this.props.allTrains).map((line: string) => {
      let visible = this.props.visibilityState.includesLine(line);
      let style = {
        background: "#" + lineColorsByName.get(line),
      };
      let className = "toggleButton " + (visible ? "active" : "inactive");
      let newVisibility = this.props.visibilityState.clone();
      newVisibility.toggleLine(line);
      let link = '/app/station/' + this.props.stationId + "/" + newVisibility.toSpec() + queryString;
      togglers.push(<div key={line} className={className} style={style}>
                      <ReactRouter.Link to={link}>{line}</ReactRouter.Link>
                    </div>);
    });

    togglers.push(<div key="sep2" className="toggleSeparator" />);

    let className = "togglebutton autowidth " + (this.props.visibilityState.isCombined() ? "active" : "inactive");
    let newVisibility = this.props.visibilityState.clone();
    newVisibility.toggleCombined();
    let link = '/app/station/' + this.props.stationId + "/" + newVisibility.toSpec() + queryString;
    togglers.push(<div key="mixing" className={className}><ReactRouter.Link to={link}>Combined</ReactRouter.Link></div>);

    if (this.props.visibilityState.shouldShowAlphaFeatures()) {
      togglers.push(<div key="sep3" className="toggleSeparator" />);
      togglers.push(<div key="makeDefault" className="togglebutton autowidth"><a href="#" onClick={this.setDefaultStationCookie.bind(this, this.props.allTrains.id)}>Make Default</a></div>);
    }

    return <div className="filterControl">
      <div key="filter" className="toggleButton autowidth"><a href="#" onClick={this.toggleExpanded.bind(this)}>Filter -</a></div>
      <div key="sep3" className="toggleSeparator" />
      {togglers}
      </div>;
  }

  public setDefaultStationCookie(stationId: string) {
    let cookieStr = "defaultStation=" + stationId + "; max-age=31536000; path=/;";
    console.log("Setting cookie: " + cookieStr);
    document.cookie = cookieStr;
  }

  private toggleExpanded() {
    this.setState({expanded: !this.state.expanded});
  }
};

export let ConnectedFilterControl = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(FilterControl);
