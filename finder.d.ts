/**
 * (node-id, is-param-node-flag, param-value)
 */
export declare type NodeTuple = [number, boolean, string];

export declare interface Finder {
  createPath(path: string): number[];
  find(path: string): NodeTuple[];
}
