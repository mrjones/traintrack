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
