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

import * as React from "react";

import * as moment from "moment";

class PubInfoState { }
class PubInfoProps {
  public reloadFn: () => void;
  public pubTimestamp: moment.Moment;
  public isLoading: boolean;
}

export class PubInfo extends React.Component<PubInfoProps, PubInfoState> {
  constructor(props: PubInfoProps) {
    super(props);
  }

  public render(): JSX.Element {
    if (!this.props.pubTimestamp || this.props.pubTimestamp.unix() === 0) {
      return <div className="pubTime">
        Waiting for data... <a href="#" onClick={this.props.reloadFn}>Force reload</a>
      </div>;
    }


    const loadingMessage = this.props.isLoading ? "[Loading...]" : "";

    return <div className="pubTime">
      Published at {this.props.pubTimestamp.format("LTS")} ({this.props.pubTimestamp.fromNow()}) <a href="#" onClick={this.props.reloadFn}>Reload</a> {loadingMessage}
      </div>;
  }
}
