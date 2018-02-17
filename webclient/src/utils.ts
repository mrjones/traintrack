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

import * as proto from './webclient_api_pb';

// TODO(mrjones): Get this from the generated file somehow
export function directionName(direction: proto.Direction): string {
  switch (direction) {
    case proto.Direction.UPTOWN:
      return "Uptown";
    case proto.Direction.DOWNTOWN:
      return "Downtown";
    default:
      console.log("Unknown Direction: " + direction);
      return "" + direction;
  }
}

export function linesForStation(station: proto.StationStatus): Immutable.OrderedSet<string> {
  let lines = Immutable.OrderedSet<string>();
  station.line.map((line: proto.LineArrivals) => {
    lines = lines.add(line.line);
  });

  return lines.sort().toOrderedSet();
}

export function directionsForStation(station: proto.StationStatus): Immutable.OrderedSet<proto.Direction> {
  let directions = Immutable.Set<proto.Direction>();
  station.line.map((line: proto.LineArrivals) => {
    directions = directions.add(line.direction);
  });

  return directions.sort().toOrderedSet();
}

export function lineVisible(line: proto.LineArrivals, lineVisibility: Immutable.Map<string, boolean>, directionVisibility: Immutable.Map<proto.Direction, boolean>): boolean {
  let lineVisible = lineVisibility.get(line.line);
  if (lineVisible === undefined) { lineVisible = true; }

  let directionVisible = directionVisibility.get(line.direction);
  if (directionVisible === undefined) { directionVisible = true; }

  return lineVisible && directionVisible;
}
