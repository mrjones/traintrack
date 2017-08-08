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
