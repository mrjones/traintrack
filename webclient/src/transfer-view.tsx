import * as Immutable from "immutable";
import * as moment from "moment";
import * as React from "react";
import * as ReactRedux from "react-redux";
import * as ReactRouter from "react-router-dom";
import * as Redux from "redux";

import * as proto from './webclient_api_pb';

import { Loadable } from './async';
import { ApiDebugger } from './debug';
import { DebuggableResult } from './datafetcher';
import { PubInfo } from './pub-info';
import { TTState } from './state-machine';
import { loadMultipleStationDetails } from './state-actions';

class TransferSpec {
  public stationId: string;
  public lines: Immutable.Set<string>;
  public direction: proto.Direction;

  // Format SSSS-LLL-D
  public static parse(serialized: string): TransferSpec | undefined {
    let parts = serialized.split("-");
    if (parts.length !== 3) {
      console.log("Invalid serialized TransferSpec: " + serialized);
      return undefined;
    }

    if (parts[2] !== "U" && parts[2] !== "D") {
      console.log("Invalid direction in TransferSpec: " + parts[2]);
      return undefined;
    }

    return {
      stationId: parts[0],
      lines: Immutable.Set(parts[1].split('')),
      direction: parts[2] === "U" ? proto.Direction.UPTOWN : proto.Direction.DOWNTOWN,
    };
  }
}

class TripWithConnections {
  public rootTs: number;
  public rootTripId: string;
  public rootStationId: string;
  public rootLine: string;
  public rootLineColorHex: string;
  // (station_id, ConnectionInfo) pair
  // ConnectionInfo can be undefined if no valid connection can be found
  public connections: [string, ConnectionInfo | undefined][];
}

class ConnectionInfo {
  public line: string;
  public inboundTimestamp: number;
  public outboundTimestamp: number;
  public waitTimeSeconds: number;
  public lineColorHex: string;
  public tripId: string;
  public stationId: string;
}

class TransferPageExplicitProps {
  public transferSpecs: string;
  public rootSpec: string;
}
class TransferPageDataProps {
  public hasAllData: boolean;
  public stationsMissingData: string[];
  public stationDatas: DebuggableResult<proto.StationStatus>[];
  public stationIndex: Map<string, number>; // index into stationDatas
}
class TransferPageDispatchProps {
  public fetchDataForStations: (stationIds: string[]) => any;
};

function rootSpecForProps(props: TransferPageExplicitProps): TransferSpec {
  return TransferSpec.parse(props.rootSpec);
}

function transferSpecsForProps(props: TransferPageExplicitProps): TransferSpec[] {
  return props.transferSpecs.split(":").map((s: string) => {
    return TransferSpec.parse(s);
  });
}

function stationIdsForProps(props: TransferPageExplicitProps): string[] {
  let stationIds = transferSpecsForProps(props).map((spec: TransferSpec) => spec.stationId);
  stationIds.push(rootSpecForProps(props).stationId);
  return stationIds;
}

const mapStateToProps = (state: TTState, ownProps: TransferPageExplicitProps): TransferPageDataProps => {
  let stationIndex = new Map<string, number>();
  let i = 0;

  let allStationIds = stationIdsForProps(ownProps);

  let maybeData: Loadable<DebuggableResult<proto.StationStatus>>[] =
    allStationIds.map((stationId: string) => {
      stationIndex.set(stationId, i++);
      return state.core.stationDetails.get(stationId);
    });

  let stationsMissingData = allStationIds.filter((stationId: string) => {
    return state.core.stationDetails.get(stationId) === undefined;
  });

  const hasAllData = maybeData.reduce((allLoaded: boolean, nextCandidate): boolean => {
    return (allLoaded && nextCandidate !== undefined && nextCandidate.valid);
  }, true);

  if (hasAllData) {
    return ({
      hasAllData: true,
      stationsMissingData: stationsMissingData,
      stationDatas: maybeData.map((loadable) => loadable.data),
      stationIndex: stationIndex,
    });
  } else {
    return ({
      hasAllData: false,
      stationsMissingData: stationsMissingData,
      stationDatas: [],
      stationIndex: new Map(),
    });
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): TransferPageDispatchProps => ({
    fetchDataForStations: (stationIds: string[]) => dispatch(loadMultipleStationDetails(stationIds)),
});

type TransferPageProps = TransferPageExplicitProps & TransferPageDataProps & TransferPageDispatchProps;
class TransferPageLocalState {
  public lastStationIdsFetched: string[];
}

class TransferPage extends React.Component<TransferPageProps, TransferPageLocalState> {
  constructor(props: TransferPageProps) {
    super(props);
    this.state = { lastStationIdsFetched: [] };
  }

  public componentWillMount() {
    this.fetchMissingData(this.props);
  }

  public componentWillReceiveProps(nextProps: TransferPageProps) {
    this.fetchMissingData(nextProps);
  }

  public render() {
    let component;
    if (!this.props.hasAllData) {
      component = <div>LOADING</div>;
    } else {
      let tripsWithConnections = this.buildTripsWithConnections();
      let lis = tripsWithConnections.map((tripWithConnections: TripWithConnections) => {
        let i = 0;
        let transferLis = tripWithConnections.connections.map((connection: [string, ConnectionInfo | undefined]) => {
          if (!connection || !connection[0]) {
            console.log("Bad connections object: " + JSON.stringify(tripWithConnections.connections));
            return <li key={i++}>No connection</li>;
          }

          if (!connection[1]) {
            return <li key={i++}>No connection at {connection[0]}</li>;
          }

          let departureTime = moment.unix(connection[1].outboundTimestamp);
          let transferArrivalTime = moment.unix(connection[1].inboundTimestamp);
          let lineStyle = {
            backgroundColor: "#" + connection[1].lineColorHex,
          };

          let durationString = connection[1].waitTimeSeconds < 120 ?
            connection[1].waitTimeSeconds + " sec" :
            Math.round(connection[1].waitTimeSeconds / 60) + " min";

          return <li key={i++}>
            <span className="lineName" style={lineStyle}>{connection[1].line}</span>
            {' '}
            {this.shortName(connection[0])}
            {' '}
            <ReactRouter.Link to={`/app/train/${tripWithConnections.rootTripId}?highlight=${tripWithConnections.rootStationId},${connection[1].stationId}`}>{transferArrivalTime.format("h:mm")}</ReactRouter.Link> - <ReactRouter.Link to={`/app/train/${connection[1].tripId}?highlight=${connection[1].stationId}`}>{departureTime.format("h:mm")}</ReactRouter.Link>
            {' '}
            (+{durationString})
            </li>;
        });

        let lineStyle = {
          backgroundColor: "#" + tripWithConnections.rootLineColorHex,
        };

        let rootTime = moment.unix(tripWithConnections.rootTs);
        return <li key={tripWithConnections.rootTripId}>
          {rootTime.format("LT")}
          {' '}
          ({rootTime.fromNow()})
          {' '}
          <span className="lineName" style={lineStyle}>{tripWithConnections.rootLine}</span>
          <ul className="transferSubtree">{transferLis}</ul>
          </li>;
      });

      let minPubTs = this.props.stationDatas.reduce(
        (minSoFar: number, candidate: DebuggableResult<proto.StationStatus>) => {
          return Math.min(minSoFar, candidate.data.dataTimestamp as number);
        }, Number.MAX_SAFE_INTEGER);

      // TODO(mrjones): Bring this back
      // <h2>{rootStation.name}</h2>
      component = <div className="transferView">
        <PubInfo pubTimestamp={moment.unix(minPubTs)} reloadFn={this.forceFetchData.bind(this)} />
        <ul className="transferTree">{lis}</ul>
      </div>;
    }

    return <div>
      {component}
      <ApiDebugger datasFetched={this.props.stationDatas} />
      </div>;
  }

  private shortName(fullName: string): string {
    let parts = fullName.split(" ");
    if (parts.length > 1 && parts[0].length < 3) {
      return parts[0] + " " + parts[1];
    }

    return parts[0];
  }

  private executeFetch(stationIds: string[]) {
    this.setState({lastStationIdsFetched: stationIds});
    this.props.fetchDataForStations(stationIds);
  }

  private forceFetchData() {
    this.executeFetch(stationIdsForProps(this.props));
  }

  private arraysEqual(a: string[], b: string[]) {
    if (a.length !== b.length) { return false; }

    for (let i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) { return false; }
    }

    return true;
  }

  private fetchMissingData(props: TransferPageProps) {
    let newStationIds = stationIdsForProps(props);

    // If the station set has changed, fetch the new data:
    if (!this.arraysEqual(newStationIds, this.state.lastStationIdsFetched)) {
      this.executeFetch(newStationIds);
    }
  }

  private buildTripsWithConnections(): TripWithConnections[] {
    let root = rootSpecForProps(this.props);
    let transfers = transferSpecsForProps(this.props);

    let rootI = this.props.stationIndex.get(root.stationId);
    let rootStation: proto.StationStatus = this.props.stationDatas[rootI].data;

    let tripsWithConnections: TripWithConnections[] = [];

    rootStation.line.forEach((line: proto.LineArrivals) => {
      if (root.lines.has(line.line) && line.direction === root.direction) {
        line.arrivals.forEach((lineArrival: proto.LineArrival) => {
          tripsWithConnections.push({
            rootStationId: root.stationId,
            rootTs: lineArrival.timestamp as number,
            rootTripId: lineArrival.tripId,
            rootLine: line.line,
            rootLineColorHex: line.lineColorHex,
            connections: this.connectionsForTrip(
              lineArrival.tripId, line.line, line.direction, transfers),
          });
        });
      }
    });

    tripsWithConnections.sort((a: TripWithConnections, b: TripWithConnections) => {
      if (a.rootTs < b.rootTs) {
        return -1;
      } else if (a.rootTs > b.rootTs) {
        return 1;
      } else {
        return 0;
      }
    });

    return tripsWithConnections;
  }

  private connectionsForTrip(
    tripId: string, line: string, direction: proto.Direction, specs: TransferSpec[]): [string, ConnectionInfo | undefined][] {
      return specs.map((spec: TransferSpec): [string, ConnectionInfo | undefined] => {
        let i = this.props.stationIndex.get(spec.stationId);
        if (i === undefined) {
          console.log("No station info for: " + spec.stationId);
          return null;
        }
        let station = this.props.stationDatas[i].data;

        let arrivalTs: number | undefined = this.findTrainArrivalTimestamp(
          station.line, line, direction, tripId);

        if (!arrivalTs) {
          return [station.name, null];
        }

        return [station.name, this.findBestConnection(
          station.line, arrivalTs, spec.lines, spec.direction, spec.stationId)];
      });
  }

  private findBestConnection(
    allTrains: proto.ILineArrivals[],
    inboundTs: number,
    outboundLines: Immutable.Set<string>,
    outboundDirection: proto.Direction,
    stationId: string): ConnectionInfo | undefined {

      let connectionInfo: ConnectionInfo | undefined = undefined;

      allTrains.map((candidateLine: proto.ILineArrivals) => {
        if (outboundLines.has(candidateLine.line) &&
            candidateLine.direction === outboundDirection) {
          for (let i = 0; i < candidateLine.arrivals.length; i++) {
            let candidate = candidateLine.arrivals[i];

            // If this candidate is after the inbound connection
            if (candidate.timestamp >= inboundTs &&
                // And either we don't have any other connection...
                (connectionInfo === undefined ||
                 // .. or this is better than our best connection so far.
                 candidate.timestamp < connectionInfo.outboundTimestamp)) {
              connectionInfo = {
                line: candidateLine.line,
                inboundTimestamp: inboundTs,
                outboundTimestamp: candidate.timestamp as number,
                waitTimeSeconds: candidate.timestamp as number - inboundTs,
                lineColorHex: candidateLine.lineColorHex,
                tripId: candidate.tripId,
                stationId: stationId,
              };
            }
          }
        }
      });

      return connectionInfo;
    }

  private findTrainArrivalTimestamp(
    allTrains: proto.ILineArrivals[],
    line: string,
    direction: proto.Direction,
    tripId: string): number | undefined {
      for (let t = 0; t < allTrains.length; t++) {
        let candidateLine = allTrains[t];
        if (candidateLine.line === line && candidateLine.direction === direction) {
          for (let a = 0; a < candidateLine.arrivals.length; a++) {
            let candidate = candidateLine.arrivals[a];
            if (candidate.tripId === tripId) {
              return candidate.timestamp as number;
            }
          }
        }
      }

      return undefined;
    }
}

export let ConnectedTransferPage = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(TransferPage);

export class TransferPageWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render() {
    console.log(JSON.stringify(this.props));
    return <ConnectedTransferPage
    rootSpec={this.props.match.params.rootSpec ? this.props.match.params.rootSpec : "028-R-U"}
    transferSpecs={this.props.match.params.transferSpec ? this.props.match.params.transferSpec : "617-D-U:026-B-U:636-AC-U"}/>;
  }
}
