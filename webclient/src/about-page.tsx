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

export class AboutPage extends React.Component<any, any> {
  constructor(props: any) {
    super(props);
  }


  public render() {
    return <div>
      <h2>About TrainTrack</h2>
      <p>TrainTrack was was developed by <a href="http://mrjon.es">Matt Jones</a>. It is written in Rust and React (source on <a href="http://github.com/mrjones/traintrack">GitHub</a>) and powered by <a href="http://datamine.mta.info/list-of-feeds">data feeds from the MTA</a>.</p>

      <p>Please send feedback to <a href="mailto:traintrack@mrjon.es">traintrack@mrjon.es</a>.</p>
      <p><b>Recent changes</b></p>
      <ul>
      <li>2018-07-02: Enabled basic data prefetching.</li>
      <li>2018-06-02: Better indication of when data is being fetched.</li>
      <li>2018-05-23: Increased file caching to speed up page loads.</li>
      <li>2018-05-22: Optimized JS file size to speed up page loads.</li>
      <li>2018-05-21: Remember recent stations in "Jump To Station" list.</li>
      <li>2018-05-21: Remember most recent station when loading home page.</li>
      <li>2018-10-09: Persist train highlighting when using station filtering controls.</li>
      <li>2019-06-05: First shot at presenting relevant service status messages for a station. Note that it is, unfortunately, still common to get irrelevant or confusing messages.</li>
      <li>2019-09-11: Added support for a new MTA API endpoint, which hopefully (?) will provide more reliable data, but could also lead to new bugs.</li>
      <li>2019-09-21: Replaced generic "Uptown" and "Downtown" labels with MTA-provided descriptions for each platform.</li>
      <li>2023-04-21: Made recently used stations show up more often.</li>
      <li>2025-01-02: Enabled filtering express lines (e.g. 7X, FX).</li>
      </ul>
      </div>;

  }


}
