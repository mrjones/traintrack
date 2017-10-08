import * as Immutable from 'immutable';

export type Loadable<T> = {
  loading: boolean;
  valid: boolean;
  data?: T;
};

export function itemIsBeingLoaded<K, L>(k: K, m: Immutable.Map<K, Loadable<L>>): boolean {
  let existing: Loadable<L> = m.get(k);
  return existing !== undefined && existing.loading;
}
