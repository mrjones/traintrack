import * as React from "react";

import * as moment from "moment";

class PubInfoState { }
class PubInfoProps {
  public reloadFn: () => void;
  public pubTimestamp: moment.Moment;
}

export class PubInfo extends React.Component<PubInfoProps, PubInfoState> {
  constructor(props: PubInfoProps) {
    super(props);
  }

  public render(): JSX.Element {
      return <div className="pubTime">
        Published at {this.props.pubTimestamp.format("LTS")} ({this.props.pubTimestamp.fromNow()}) <a href="#" onClick={this.props.reloadFn}>Reload</a>
      </div>;
  }
}
