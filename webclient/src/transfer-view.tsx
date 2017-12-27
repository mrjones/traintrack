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

class ConnectionInfo {
  public line: string;
  public inboundTimestamp: number;
  public outboundTimestamp: number;
  public waitTimeSeconds: number;
  public lineColorHex: string;
}

class TransferPageExplicitProps {
  public transferSpecs: string;
  public rootSpec: string;
}
class TransferPageDataProps {
  public hasData: boolean;
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

  let maybeData: Loadable<DebuggableResult<proto.StationStatus>>[] =
    stationIdsForProps(ownProps).map((stationId: string) => {
      stationIndex.set(stationId, i++);
      return state.core.stationDetails.get(stationId);
    });

  const hasData = maybeData.reduce((allLoaded: boolean, nextCandidate): boolean => {
    return (allLoaded && nextCandidate !== undefined && nextCandidate.valid);
  }, true);

  if (hasData) {
    return ({
      hasData: true,
      stationDatas: maybeData.map((loadable) => loadable.data),
      stationIndex: stationIndex,
    });
  } else {
    return ({
      hasData: false,
      stationDatas: [],
      stationIndex: new Map(),
    });
  }
};

const mapDispatchToProps = (dispatch: Redux.Dispatch<TTState>): TransferPageDispatchProps => ({
    fetchDataForStations: (stationIds: string[]) => dispatch(loadMultipleStationDetails(stationIds)),
});

type TransferPageProps = TransferPageExplicitProps & TransferPageDataProps & TransferPageDispatchProps;
class TransferPageLocalState { }

class TransferPage extends React.Component<TransferPageProps, TransferPageLocalState> {
  constructor(props: TransferPageProps) {
    super(props);
  }

  public componentWillMount() {
    this.fetchMissingData(this.props);
  }

  public componentWillReceiveProps(nextProps: TransferPageProps) {
    this.fetchMissingData(nextProps);
  }

  public render() {
    let root = rootSpecForProps(this.props);
    let transfers = transferSpecsForProps(this.props);
    /*
    let root: TransferSpec = {
      stationId: "028",
      lines: Immutable.Set.of("R"),
      direction: proto.Direction.UPTOWN,
    };

    let transfers: TransferSpec[] = [
      { stationId: "617", lines: Immutable.Set.of("D"), direction: proto.Direction.UPTOWN },
      { stationId: "026", lines: Immutable.Set.of("B"), direction: proto.Direction.UPTOWN },
      { stationId: "636", lines: Immutable.Set.of("A", "C"), direction: proto.Direction.UPTOWN },
    ];
    */

    let component;
    if (!this.props.hasData) {
      component = <div>LOADING</div>;
    } else {
      let rootI = this.props.stationIndex.get(root.stationId);

      let rootStation: proto.StationStatus = this.props.stationDatas[rootI].data;

      let lis;
      rootStation.line.forEach((line: proto.LineArrivals) => {
        if (root.lines.has(line.line) && line.direction === root.direction) {
          lis = line.arrivals.map((lineArrival: proto.LineArrival) => {
            const rootTs = lineArrival.timestamp as number;
            const rootTrip = lineArrival.tripId;
            const rootTime = moment.unix(rootTs);

            let connections = this.connectionsForTrip(rootTrip, line.line, line.direction, transfers);

            let transferLis = connections.map((connection: [string, ConnectionInfo | undefined]) => {
              if (connection[1]) {
                let departureTime = moment.unix(connection[1].outboundTimestamp);
                let waitDuration = moment.duration(connection[1].waitTimeSeconds, 'seconds');
                let transferArrivalTime = moment.unix(connection[1].inboundTimestamp);
                let lineStyle = {
                  backgroundColor: "#" + connection[1].lineColorHex,
                };
                return <li>
                  <span className="lineName" style={lineStyle}>{connection[1].line}</span>&nbsp;
                  {this.shortName(connection[0])}&nbsp;
                  {transferArrivalTime.format("LT")} --&gt; {departureTime.format("LT")}&nbsp;
                  (+ {waitDuration.locale("en").humanize()})
                </li>;
              } else {
                return <li>No connectionion at {connection[0]}</li>;
              }
            });

            return <li>{rootTime.format("LT")} {line.line} ({rootTime.fromNow()})
              <ul className="transferSubtree">{transferLis}</ul>
              </li>;
          });
        }
      });

      let minPubTs = this.props.stationDatas.reduce(
        (minSoFar: number, candidate: DebuggableResult<proto.StationStatus>) => {
          return Math.min(minSoFar, candidate.data.dataTimestamp as number);
        }, Number.MAX_SAFE_INTEGER);

      component = <div className="transferView">
        <h2>{rootStation.name}</h2>
        <PubInfo pubTimestamp={moment.unix(minPubTs)} reloadFn={this.fetchData.bind(this)} />
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

  private fetchData() {
    this.props.fetchDataForStations(stationIdsForProps(this.props));
  }

  private fetchMissingData(props: TransferPageProps) {
    if (!props.hasData) {
      this.props.fetchDataForStations(stationIdsForProps(props));
    }
  }

  private connectionsForTrip(
    tripId: string, line: string, direction: proto.Direction, specs: TransferSpec[]): [string, ConnectionInfo | undefined][] {
      return specs.map((spec: TransferSpec): [string, ConnectionInfo | undefined] => {
        let i = this.props.stationIndex.get(spec.stationId);
        if (i === undefined) { return null; }
        let station = this.props.stationDatas[i].data;

        let arrivalTs: number | undefined = this.findTrainArrivalTimestamp(
          station.line, line, direction, tripId);

        if (!arrivalTs) { return undefined; }

        return [station.name, this.findBestConnection(
          station.line, arrivalTs, spec.lines, spec.direction)];
      });
  }

  private findBestConnection(
    allTrains: proto.ILineArrivals[],
    inboundTs: number,
    outboundLines: Immutable.Set<string>,
    outboundDirection: proto.Direction): ConnectionInfo | undefined {

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
