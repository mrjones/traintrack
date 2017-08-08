import * as React from "react";
import * as proto from './webclient_api_pb';

import * as utils from './utils'

class FilterControlProps {
  public interestingDirections: proto.Direction[];
  public updateFilterPredicateFn: (newFn: (l: proto.LineArrivals) => boolean) => void;
};

class FilterControlState {
  public directionStates: Map<proto.Direction, boolean>;
};

export class FilterControl extends React.Component<FilterControlProps, FilterControlState> {
  public constructor(props: FilterControlProps) {
    super(props);

    let directionStates = new Map<proto.Direction, boolean>();
    props.interestingDirections.map((direction: proto.Direction) => {
      directionStates.set(direction, true);
    });

    this.state = {
      directionStates: directionStates,
    };

  }

  public render(): JSX.Element {
    let directionTogglers = new Array<JSX.Element>();
    this.state.directionStates.forEach((visible: boolean, direction: proto.Direction) => {
      directionTogglers.push(<a href="#" onClick={this.toggleDirection.bind(this, direction)}>Toggle {utils.directionName(direction)}</a>);
    });

    return <div>
      Filter control
      {directionTogglers}
      </div>;
  }

  public componentDidMount() {
    this.props.updateFilterPredicateFn(this.filterPredicate.bind(this));
  }

  private filterPredicate(line: proto.LineArrivals): boolean {
    const dState = this.state.directionStates.get(line.direction);
    if (dState === undefined) {
      console.log("Unknown direction: " + utils.directionName(line.direction));
      return true;
    } else {
      return dState;
    }
  }

  private toggleDirection(direction: proto.Direction) {
    const dState = this.state.directionStates.get(direction);
    if (dState === undefined) {
      console.log("Unknown direction: " + utils.directionName(direction));
      this.state.directionStates.set(direction, false);
    } else {
      this.state.directionStates.set(direction, !dState);
    }
  };
};
