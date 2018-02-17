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

declare module 'react-g-analytics' {
  import * as ReactRouter from "react-router-dom";
  import * as React from "react";

  interface BrowserRouterProps {
    // From ReactRouter.BrowserRouterProps
    basename?: string;
    getUserConfirmation?(): void;
    forceRefresh?: boolean;
    keyLength?: number;

    // react-g-analytics extension
    id: string;
  }

  class BrowserRouter extends React.Component<BrowserRouterProps> { }
}
