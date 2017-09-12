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

export function linesForStation(station: proto.StationStatus): Immutable.Set<string> {
  let lines = Immutable.Set<string>();
  station.line.map((line: proto.LineArrivals) => {
    lines = lines.add(line.line);
  });

  return lines;
}

export function lineVisible(line: proto.LineArrivals, lineVisibility: Immutable.Map<string, boolean>): boolean {
  let visible = lineVisibility.get(line.line);
  if (visible === undefined) { visible = true; }
  return visible;
}
