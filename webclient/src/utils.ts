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
