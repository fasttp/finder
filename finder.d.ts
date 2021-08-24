export declare type NodeId = number;
export declare type IsParam = boolean;
export declare type ParamValue = string;

export declare type NodeTuple = [NodeId, IsParam, ParamValue];

export declare class Finder {
  createPath(path: string): number[];
  find(path: string): NodeTuple[];
}
