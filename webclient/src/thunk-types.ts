import * as ReduxThunk from "redux-thunk";
import * as Redux from "redux";

import { TTContext, TTState } from './state-machine';

export type TTAsyncThunkAction = ReduxThunk.ThunkAction<Promise<Redux.Action>, TTState, TTContext, Redux.Action>;

export type TTThunkDispatch = ReduxThunk.ThunkDispatch<TTState, TTContext, Redux.Action>;
