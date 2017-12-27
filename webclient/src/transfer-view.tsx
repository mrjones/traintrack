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

class TransferPageExplicitProps {
  public stationIds: string[];
}
class TransferPageDataProps {
  public hasData: boolean;
  public stationDatas: DebuggableResult<proto.StationStatus>[];
  public stationIndex: Map<string, number>; // index into stationDatas
}
class TransferPageDispatchProps {
  public fetchDataForStations: (stationIds: string[]) => any;
};

const mapStateToProps = (state: TTState, ownProps: TransferPageExplicitProps): TransferPageDataProps => {
  let stationIndex = new Map<string, number>();
  let i = 0;

  let maybeData: Loadable<DebuggableResult<proto.StationStatus>>[] =
    ownProps.stationIds.map((stationId: string) => {
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
    let root: [string, Immutable.Set<string>, proto.Direction] =
      ["028", Immutable.Set.of("R"), proto.Direction.UPTOWN];
    let leaves: [string, Immutable.Set<string>, proto.Direction][] = [
      ["617", Immutable.Set.of("D"), proto.Direction.UPTOWN],
      ["026", Immutable.Set.of("B"), proto.Direction.UPTOWN],
      ["636", Immutable.Set.of("A", "C"), proto.Direction.UPTOWN],
    ];

    let component;
    if (!this.props.hasData) {
      component = <div>LOADING</div>;
    } else {
      let rootI = this.props.stationIndex.get(root[0]);

      let rootStation: proto.StationStatus = this.props.stationDatas[rootI].data;

      let lis;
      rootStation.line.forEach((line: proto.LineArrivals) => {
        if (root[1].has(line.line) && line.direction === root[2]) {
          lis = line.arrivals.map((lineArrival: proto.LineArrival) => {
            const rootTs = lineArrival.timestamp as number;
            const rootTrip = lineArrival.tripId;
            const rootTime = moment.unix(rootTs);

            let leafLis = leaves.map((leafSpec) => {
              let leafI = this.props.stationIndex.get(leafSpec[0]);
              if (leafI === undefined) { return null; }
              let leafStation = this.props.stationDatas[leafI].data;

              let leafArrivalTs = 0;
              let leafArrivalTime = moment.unix(0);

              leafStation.line.map((leafLine: proto.LineArrivals) => {
                if (root[1].has(line.line) && leafLine.direction === root[2]) {
                  for (let i = 0; i < leafLine.arrivals.length; i++) {
                    if (leafLine.arrivals[i].tripId === rootTrip) {
                      leafArrivalTs = leafLine.arrivals[i].timestamp as number;
                      leafArrivalTime = moment.unix(leafArrivalTs);
                      break;
                    }
                  }
                }
              });

              let maybeConn = undefined;
              if (leafArrivalTs > 0) {
                maybeConn = this.findBestConnection(
                  leafStation.line, leafArrivalTs, leafSpec[1], leafSpec[2]);
              }
              if (maybeConn) {
                let departureTime = moment.unix(maybeConn.timestamp);
                let waitDuration = moment.duration(maybeConn.waitTimeSeconds, 'seconds');
                return <li>{leafStation.name.split(" ")[0]} {leafArrivalTime.format("LT")} --&gt; {maybeConn.line} at {departureTime.format("LT")} (+ {waitDuration.locale("en").humanize()})</li>;
              } else {
                return <li>No matching train at {leafStation.name}</li>;
              }
            });

            return <li>{rootTime.format("LT")} ({rootTime.fromNow()})<ul>{leafLis}</ul></li>;
          });
        }
      });

      component = <div>{rootStation.name}<ul>{lis}</ul></div>;
    }

    let minTs = this.props.stationDatas.reduce(
      (minSoFar: number, candidate: DebuggableResult<proto.StationStatus>) => {
        return Math.min(minSoFar, candidate.data.dataTimestamp as number);
      }, Number.MAX_SAFE_INTEGER);

    return <div>
      <PubInfo pubTimestamp={moment.unix(minTs)} reloadFn={this.fetchData.bind(this)} />
      {component}
      <ApiDebugger datasFetched={this.props.stationDatas} />
      </div>;
  }

  private fetchData() {
    this.props.fetchDataForStations(this.props.stationIds);
  }

  private fetchMissingData(props: TransferPageProps) {
    if (!props.hasData) {
      this.props.fetchDataForStations(props.stationIds);
    }
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
                 candidate.timestamp < connectionInfo.timestamp)) {
              connectionInfo = {
                line: candidateLine.line,
                timestamp: candidate.timestamp as number,
                waitTimeSeconds: candidate.timestamp as number - inboundTs,
              };
            }
          }
        }
      });

      return connectionInfo;
    }
}

class ConnectionInfo {
  public line: string;
  public timestamp: number;
  public waitTimeSeconds: number;
}

export let ConnectedTransferPage = ReactRedux.connect(mapStateToProps, mapDispatchToProps)(TransferPage);

export class TransferPageWrapper extends React.Component<ReactRouter.RouteComponentProps<any>, any> {
  constructor(props: ReactRouter.RouteComponentProps<any>) {
    super(props);
  }

  public render() {
    return <ConnectedTransferPage stationIds={["026", "028", "617", "636"]} />;
  }
}
