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

import * as $protobuf from "protobufjs";
/** Direction enum. */
export enum Direction {
    UPTOWN = 0,
    DOWNTOWN = 1
}

/** Properties of a DebugInfo. */
export interface IDebugInfo {

    /** DebugInfo processingTimeMs */
    processingTimeMs?: (number|Long|null);

    /** DebugInfo buildVersion */
    buildVersion?: (string|null);

    /** DebugInfo buildTimestamp */
    buildTimestamp?: (number|Long|null);
}

/** Represents a DebugInfo. */
export class DebugInfo implements IDebugInfo {

    /**
     * Constructs a new DebugInfo.
     * @param [properties] Properties to set
     */
    constructor(properties?: IDebugInfo);

    /** DebugInfo processingTimeMs. */
    public processingTimeMs: (number|Long);

    /** DebugInfo buildVersion. */
    public buildVersion: string;

    /** DebugInfo buildTimestamp. */
    public buildTimestamp: (number|Long);

    /**
     * Creates a new DebugInfo instance using the specified properties.
     * @param [properties] Properties to set
     * @returns DebugInfo instance
     */
    public static create(properties?: IDebugInfo): DebugInfo;

    /**
     * Encodes the specified DebugInfo message. Does not implicitly {@link DebugInfo.verify|verify} messages.
     * @param message DebugInfo message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IDebugInfo, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified DebugInfo message, length delimited. Does not implicitly {@link DebugInfo.verify|verify} messages.
     * @param message DebugInfo message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IDebugInfo, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a DebugInfo message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns DebugInfo
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): DebugInfo;

    /**
     * Decodes a DebugInfo message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns DebugInfo
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): DebugInfo;

    /**
     * Verifies a DebugInfo message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a DebugInfo message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns DebugInfo
     */
    public static fromObject(object: { [k: string]: any }): DebugInfo;

    /**
     * Creates a plain object from a DebugInfo message. Also converts values to other types if specified.
     * @param message DebugInfo
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: DebugInfo, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this DebugInfo to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a LineArrival. */
export interface ILineArrival {

    /** LineArrival timestamp */
    timestamp?: (number|Long|null);

    /** LineArrival tripId */
    tripId?: (string|null);

    /** LineArrival headsign */
    headsign?: (string|null);
}

/** Represents a LineArrival. */
export class LineArrival implements ILineArrival {

    /**
     * Constructs a new LineArrival.
     * @param [properties] Properties to set
     */
    constructor(properties?: ILineArrival);

    /** LineArrival timestamp. */
    public timestamp: (number|Long);

    /** LineArrival tripId. */
    public tripId: string;

    /** LineArrival headsign. */
    public headsign: string;

    /**
     * Creates a new LineArrival instance using the specified properties.
     * @param [properties] Properties to set
     * @returns LineArrival instance
     */
    public static create(properties?: ILineArrival): LineArrival;

    /**
     * Encodes the specified LineArrival message. Does not implicitly {@link LineArrival.verify|verify} messages.
     * @param message LineArrival message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ILineArrival, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified LineArrival message, length delimited. Does not implicitly {@link LineArrival.verify|verify} messages.
     * @param message LineArrival message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ILineArrival, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a LineArrival message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns LineArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): LineArrival;

    /**
     * Decodes a LineArrival message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns LineArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): LineArrival;

    /**
     * Verifies a LineArrival message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a LineArrival message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns LineArrival
     */
    public static fromObject(object: { [k: string]: any }): LineArrival;

    /**
     * Creates a plain object from a LineArrival message. Also converts values to other types if specified.
     * @param message LineArrival
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: LineArrival, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this LineArrival to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a LineArrivals. */
export interface ILineArrivals {

    /** LineArrivals line */
    line?: (string|null);

    /** LineArrivals direction */
    direction?: (Direction|null);

    /** LineArrivals lineColorHex */
    lineColorHex?: (string|null);

    /** LineArrivals arrivals */
    arrivals?: (ILineArrival[]|null);

    /** LineArrivals debugInfo */
    debugInfo?: (IDebugInfo|null);
}

/** Represents a LineArrivals. */
export class LineArrivals implements ILineArrivals {

    /**
     * Constructs a new LineArrivals.
     * @param [properties] Properties to set
     */
    constructor(properties?: ILineArrivals);

    /** LineArrivals line. */
    public line: string;

    /** LineArrivals direction. */
    public direction: Direction;

    /** LineArrivals lineColorHex. */
    public lineColorHex: string;

    /** LineArrivals arrivals. */
    public arrivals: ILineArrival[];

    /** LineArrivals debugInfo. */
    public debugInfo?: (IDebugInfo|null);

    /**
     * Creates a new LineArrivals instance using the specified properties.
     * @param [properties] Properties to set
     * @returns LineArrivals instance
     */
    public static create(properties?: ILineArrivals): LineArrivals;

    /**
     * Encodes the specified LineArrivals message. Does not implicitly {@link LineArrivals.verify|verify} messages.
     * @param message LineArrivals message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ILineArrivals, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified LineArrivals message, length delimited. Does not implicitly {@link LineArrivals.verify|verify} messages.
     * @param message LineArrivals message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ILineArrivals, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a LineArrivals message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns LineArrivals
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): LineArrivals;

    /**
     * Decodes a LineArrivals message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns LineArrivals
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): LineArrivals;

    /**
     * Verifies a LineArrivals message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a LineArrivals message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns LineArrivals
     */
    public static fromObject(object: { [k: string]: any }): LineArrivals;

    /**
     * Creates a plain object from a LineArrivals message. Also converts values to other types if specified.
     * @param message LineArrivals
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: LineArrivals, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this LineArrivals to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TrainItineraryArrival. */
export interface ITrainItineraryArrival {

    /** TrainItineraryArrival timestamp */
    timestamp?: (number|Long|null);

    /** TrainItineraryArrival station */
    station?: (IStation|null);
}

/** Represents a TrainItineraryArrival. */
export class TrainItineraryArrival implements ITrainItineraryArrival {

    /**
     * Constructs a new TrainItineraryArrival.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITrainItineraryArrival);

    /** TrainItineraryArrival timestamp. */
    public timestamp: (number|Long);

    /** TrainItineraryArrival station. */
    public station?: (IStation|null);

    /**
     * Creates a new TrainItineraryArrival instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TrainItineraryArrival instance
     */
    public static create(properties?: ITrainItineraryArrival): TrainItineraryArrival;

    /**
     * Encodes the specified TrainItineraryArrival message. Does not implicitly {@link TrainItineraryArrival.verify|verify} messages.
     * @param message TrainItineraryArrival message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITrainItineraryArrival, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TrainItineraryArrival message, length delimited. Does not implicitly {@link TrainItineraryArrival.verify|verify} messages.
     * @param message TrainItineraryArrival message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITrainItineraryArrival, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TrainItineraryArrival message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TrainItineraryArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TrainItineraryArrival;

    /**
     * Decodes a TrainItineraryArrival message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TrainItineraryArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TrainItineraryArrival;

    /**
     * Verifies a TrainItineraryArrival message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TrainItineraryArrival message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TrainItineraryArrival
     */
    public static fromObject(object: { [k: string]: any }): TrainItineraryArrival;

    /**
     * Creates a plain object from a TrainItineraryArrival message. Also converts values to other types if specified.
     * @param message TrainItineraryArrival
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TrainItineraryArrival, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TrainItineraryArrival to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TrainItinerary. */
export interface ITrainItinerary {

    /** TrainItinerary line */
    line?: (string|null);

    /** TrainItinerary direction */
    direction?: (Direction|null);

    /** TrainItinerary lineColorHex */
    lineColorHex?: (string|null);

    /** TrainItinerary arrival */
    arrival?: (ITrainItineraryArrival[]|null);

    /** TrainItinerary dataTimestamp */
    dataTimestamp?: (number|Long|null);

    /** TrainItinerary debugInfo */
    debugInfo?: (IDebugInfo|null);
}

/** Represents a TrainItinerary. */
export class TrainItinerary implements ITrainItinerary {

    /**
     * Constructs a new TrainItinerary.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITrainItinerary);

    /** TrainItinerary line. */
    public line: string;

    /** TrainItinerary direction. */
    public direction: Direction;

    /** TrainItinerary lineColorHex. */
    public lineColorHex: string;

    /** TrainItinerary arrival. */
    public arrival: ITrainItineraryArrival[];

    /** TrainItinerary dataTimestamp. */
    public dataTimestamp: (number|Long);

    /** TrainItinerary debugInfo. */
    public debugInfo?: (IDebugInfo|null);

    /**
     * Creates a new TrainItinerary instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TrainItinerary instance
     */
    public static create(properties?: ITrainItinerary): TrainItinerary;

    /**
     * Encodes the specified TrainItinerary message. Does not implicitly {@link TrainItinerary.verify|verify} messages.
     * @param message TrainItinerary message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITrainItinerary, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TrainItinerary message, length delimited. Does not implicitly {@link TrainItinerary.verify|verify} messages.
     * @param message TrainItinerary message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITrainItinerary, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TrainItinerary message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TrainItinerary
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TrainItinerary;

    /**
     * Decodes a TrainItinerary message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TrainItinerary
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TrainItinerary;

    /**
     * Verifies a TrainItinerary message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TrainItinerary message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TrainItinerary
     */
    public static fromObject(object: { [k: string]: any }): TrainItinerary;

    /**
     * Creates a plain object from a TrainItinerary message. Also converts values to other types if specified.
     * @param message TrainItinerary
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TrainItinerary, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TrainItinerary to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a StationStatus. */
export interface IStationStatus {

    /** StationStatus name */
    name?: (string|null);

    /** StationStatus id */
    id?: (string|null);

    /** StationStatus line */
    line?: (ILineArrivals[]|null);

    /** StationStatus dataTimestamp */
    dataTimestamp?: (number|Long|null);

    /** StationStatus statusMessage */
    statusMessage?: (ISubwayStatusMessage[]|null);

    /** StationStatus debugInfo */
    debugInfo?: (IDebugInfo|null);
}

/** Represents a StationStatus. */
export class StationStatus implements IStationStatus {

    /**
     * Constructs a new StationStatus.
     * @param [properties] Properties to set
     */
    constructor(properties?: IStationStatus);

    /** StationStatus name. */
    public name: string;

    /** StationStatus id. */
    public id: string;

    /** StationStatus line. */
    public line: ILineArrivals[];

    /** StationStatus dataTimestamp. */
    public dataTimestamp: (number|Long);

    /** StationStatus statusMessage. */
    public statusMessage: ISubwayStatusMessage[];

    /** StationStatus debugInfo. */
    public debugInfo?: (IDebugInfo|null);

    /**
     * Creates a new StationStatus instance using the specified properties.
     * @param [properties] Properties to set
     * @returns StationStatus instance
     */
    public static create(properties?: IStationStatus): StationStatus;

    /**
     * Encodes the specified StationStatus message. Does not implicitly {@link StationStatus.verify|verify} messages.
     * @param message StationStatus message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IStationStatus, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified StationStatus message, length delimited. Does not implicitly {@link StationStatus.verify|verify} messages.
     * @param message StationStatus message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IStationStatus, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a StationStatus message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns StationStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): StationStatus;

    /**
     * Decodes a StationStatus message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns StationStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): StationStatus;

    /**
     * Verifies a StationStatus message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a StationStatus message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns StationStatus
     */
    public static fromObject(object: { [k: string]: any }): StationStatus;

    /**
     * Creates a plain object from a StationStatus message. Also converts values to other types if specified.
     * @param message StationStatus
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: StationStatus, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this StationStatus to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a Station. */
export interface IStation {

    /** Station id */
    id?: (string|null);

    /** Station name */
    name?: (string|null);

    /** Station lines */
    lines?: (string[]|null);
}

/** Represents a Station. */
export class Station implements IStation {

    /**
     * Constructs a new Station.
     * @param [properties] Properties to set
     */
    constructor(properties?: IStation);

    /** Station id. */
    public id: string;

    /** Station name. */
    public name: string;

    /** Station lines. */
    public lines: string[];

    /**
     * Creates a new Station instance using the specified properties.
     * @param [properties] Properties to set
     * @returns Station instance
     */
    public static create(properties?: IStation): Station;

    /**
     * Encodes the specified Station message. Does not implicitly {@link Station.verify|verify} messages.
     * @param message Station message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IStation, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified Station message, length delimited. Does not implicitly {@link Station.verify|verify} messages.
     * @param message Station message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IStation, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a Station message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns Station
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): Station;

    /**
     * Decodes a Station message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns Station
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): Station;

    /**
     * Verifies a Station message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a Station message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns Station
     */
    public static fromObject(object: { [k: string]: any }): Station;

    /**
     * Creates a plain object from a Station message. Also converts values to other types if specified.
     * @param message Station
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: Station, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this Station to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a StationList. */
export interface IStationList {

    /** StationList station */
    station?: (IStation[]|null);

    /** StationList debugInfo */
    debugInfo?: (IDebugInfo|null);
}

/** Represents a StationList. */
export class StationList implements IStationList {

    /**
     * Constructs a new StationList.
     * @param [properties] Properties to set
     */
    constructor(properties?: IStationList);

    /** StationList station. */
    public station: IStation[];

    /** StationList debugInfo. */
    public debugInfo?: (IDebugInfo|null);

    /**
     * Creates a new StationList instance using the specified properties.
     * @param [properties] Properties to set
     * @returns StationList instance
     */
    public static create(properties?: IStationList): StationList;

    /**
     * Encodes the specified StationList message. Does not implicitly {@link StationList.verify|verify} messages.
     * @param message StationList message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IStationList, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified StationList message, length delimited. Does not implicitly {@link StationList.verify|verify} messages.
     * @param message StationList message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IStationList, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a StationList message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns StationList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): StationList;

    /**
     * Decodes a StationList message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns StationList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): StationList;

    /**
     * Verifies a StationList message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a StationList message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns StationList
     */
    public static fromObject(object: { [k: string]: any }): StationList;

    /**
     * Creates a plain object from a StationList message. Also converts values to other types if specified.
     * @param message StationList
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: StationList, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this StationList to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a Line. */
export interface ILine {

    /** Line name */
    name?: (string|null);

    /** Line colorHex */
    colorHex?: (string|null);

    /** Line active */
    active?: (boolean|null);
}

/** Represents a Line. */
export class Line implements ILine {

    /**
     * Constructs a new Line.
     * @param [properties] Properties to set
     */
    constructor(properties?: ILine);

    /** Line name. */
    public name: string;

    /** Line colorHex. */
    public colorHex: string;

    /** Line active. */
    public active: boolean;

    /**
     * Creates a new Line instance using the specified properties.
     * @param [properties] Properties to set
     * @returns Line instance
     */
    public static create(properties?: ILine): Line;

    /**
     * Encodes the specified Line message. Does not implicitly {@link Line.verify|verify} messages.
     * @param message Line message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ILine, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified Line message, length delimited. Does not implicitly {@link Line.verify|verify} messages.
     * @param message Line message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ILine, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a Line message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns Line
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): Line;

    /**
     * Decodes a Line message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns Line
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): Line;

    /**
     * Verifies a Line message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a Line message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns Line
     */
    public static fromObject(object: { [k: string]: any }): Line;

    /**
     * Creates a plain object from a Line message. Also converts values to other types if specified.
     * @param message Line
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: Line, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this Line to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a LineList. */
export interface ILineList {

    /** LineList line */
    line?: (ILine[]|null);

    /** LineList debugInfo */
    debugInfo?: (IDebugInfo|null);
}

/** Represents a LineList. */
export class LineList implements ILineList {

    /**
     * Constructs a new LineList.
     * @param [properties] Properties to set
     */
    constructor(properties?: ILineList);

    /** LineList line. */
    public line: ILine[];

    /** LineList debugInfo. */
    public debugInfo?: (IDebugInfo|null);

    /**
     * Creates a new LineList instance using the specified properties.
     * @param [properties] Properties to set
     * @returns LineList instance
     */
    public static create(properties?: ILineList): LineList;

    /**
     * Encodes the specified LineList message. Does not implicitly {@link LineList.verify|verify} messages.
     * @param message LineList message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ILineList, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified LineList message, length delimited. Does not implicitly {@link LineList.verify|verify} messages.
     * @param message LineList message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ILineList, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a LineList message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns LineList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): LineList;

    /**
     * Decodes a LineList message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns LineList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): LineList;

    /**
     * Verifies a LineList message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a LineList message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns LineList
     */
    public static fromObject(object: { [k: string]: any }): LineList;

    /**
     * Creates a plain object from a LineList message. Also converts values to other types if specified.
     * @param message LineList
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: LineList, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this LineList to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TrainArrivalHistoryEntry. */
export interface ITrainArrivalHistoryEntry {

    /** TrainArrivalHistoryEntry dataTimestamp */
    dataTimestamp?: (number|Long|null);

    /** TrainArrivalHistoryEntry arrivalTime */
    arrivalTime?: (number|Long|null);
}

/** Represents a TrainArrivalHistoryEntry. */
export class TrainArrivalHistoryEntry implements ITrainArrivalHistoryEntry {

    /**
     * Constructs a new TrainArrivalHistoryEntry.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITrainArrivalHistoryEntry);

    /** TrainArrivalHistoryEntry dataTimestamp. */
    public dataTimestamp: (number|Long);

    /** TrainArrivalHistoryEntry arrivalTime. */
    public arrivalTime: (number|Long);

    /**
     * Creates a new TrainArrivalHistoryEntry instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TrainArrivalHistoryEntry instance
     */
    public static create(properties?: ITrainArrivalHistoryEntry): TrainArrivalHistoryEntry;

    /**
     * Encodes the specified TrainArrivalHistoryEntry message. Does not implicitly {@link TrainArrivalHistoryEntry.verify|verify} messages.
     * @param message TrainArrivalHistoryEntry message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITrainArrivalHistoryEntry, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TrainArrivalHistoryEntry message, length delimited. Does not implicitly {@link TrainArrivalHistoryEntry.verify|verify} messages.
     * @param message TrainArrivalHistoryEntry message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITrainArrivalHistoryEntry, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TrainArrivalHistoryEntry message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TrainArrivalHistoryEntry
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TrainArrivalHistoryEntry;

    /**
     * Decodes a TrainArrivalHistoryEntry message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TrainArrivalHistoryEntry
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TrainArrivalHistoryEntry;

    /**
     * Verifies a TrainArrivalHistoryEntry message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TrainArrivalHistoryEntry message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TrainArrivalHistoryEntry
     */
    public static fromObject(object: { [k: string]: any }): TrainArrivalHistoryEntry;

    /**
     * Creates a plain object from a TrainArrivalHistoryEntry message. Also converts values to other types if specified.
     * @param message TrainArrivalHistoryEntry
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TrainArrivalHistoryEntry, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TrainArrivalHistoryEntry to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a TrainArrivalHistory. */
export interface ITrainArrivalHistory {

    /** TrainArrivalHistory debugInfo */
    debugInfo?: (IDebugInfo|null);

    /** TrainArrivalHistory history */
    history?: (ITrainArrivalHistoryEntry[]|null);
}

/** Represents a TrainArrivalHistory. */
export class TrainArrivalHistory implements ITrainArrivalHistory {

    /**
     * Constructs a new TrainArrivalHistory.
     * @param [properties] Properties to set
     */
    constructor(properties?: ITrainArrivalHistory);

    /** TrainArrivalHistory debugInfo. */
    public debugInfo?: (IDebugInfo|null);

    /** TrainArrivalHistory history. */
    public history: ITrainArrivalHistoryEntry[];

    /**
     * Creates a new TrainArrivalHistory instance using the specified properties.
     * @param [properties] Properties to set
     * @returns TrainArrivalHistory instance
     */
    public static create(properties?: ITrainArrivalHistory): TrainArrivalHistory;

    /**
     * Encodes the specified TrainArrivalHistory message. Does not implicitly {@link TrainArrivalHistory.verify|verify} messages.
     * @param message TrainArrivalHistory message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ITrainArrivalHistory, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified TrainArrivalHistory message, length delimited. Does not implicitly {@link TrainArrivalHistory.verify|verify} messages.
     * @param message TrainArrivalHistory message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ITrainArrivalHistory, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a TrainArrivalHistory message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns TrainArrivalHistory
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): TrainArrivalHistory;

    /**
     * Decodes a TrainArrivalHistory message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns TrainArrivalHistory
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): TrainArrivalHistory;

    /**
     * Verifies a TrainArrivalHistory message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a TrainArrivalHistory message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns TrainArrivalHistory
     */
    public static fromObject(object: { [k: string]: any }): TrainArrivalHistory;

    /**
     * Creates a plain object from a TrainArrivalHistory message. Also converts values to other types if specified.
     * @param message TrainArrivalHistory
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: TrainArrivalHistory, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this TrainArrivalHistory to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of a SubwayStatusMessage. */
export interface ISubwayStatusMessage {

    /** SubwayStatusMessage summary */
    summary?: (string|null);

    /** SubwayStatusMessage longDescription */
    longDescription?: (string|null);

    /** SubwayStatusMessage affectedLine */
    affectedLine?: (IAffectedLineStatus[]|null);

    /** SubwayStatusMessage planned */
    planned?: (boolean|null);

    /** SubwayStatusMessage reasonName */
    reasonName?: (string|null);

    /** SubwayStatusMessage priority */
    priority?: (number|null);

    /** SubwayStatusMessage publishTimestamp */
    publishTimestamp?: (number|Long|null);
}

/** Represents a SubwayStatusMessage. */
export class SubwayStatusMessage implements ISubwayStatusMessage {

    /**
     * Constructs a new SubwayStatusMessage.
     * @param [properties] Properties to set
     */
    constructor(properties?: ISubwayStatusMessage);

    /** SubwayStatusMessage summary. */
    public summary: string;

    /** SubwayStatusMessage longDescription. */
    public longDescription: string;

    /** SubwayStatusMessage affectedLine. */
    public affectedLine: IAffectedLineStatus[];

    /** SubwayStatusMessage planned. */
    public planned: boolean;

    /** SubwayStatusMessage reasonName. */
    public reasonName: string;

    /** SubwayStatusMessage priority. */
    public priority: number;

    /** SubwayStatusMessage publishTimestamp. */
    public publishTimestamp: (number|Long);

    /**
     * Creates a new SubwayStatusMessage instance using the specified properties.
     * @param [properties] Properties to set
     * @returns SubwayStatusMessage instance
     */
    public static create(properties?: ISubwayStatusMessage): SubwayStatusMessage;

    /**
     * Encodes the specified SubwayStatusMessage message. Does not implicitly {@link SubwayStatusMessage.verify|verify} messages.
     * @param message SubwayStatusMessage message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: ISubwayStatusMessage, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified SubwayStatusMessage message, length delimited. Does not implicitly {@link SubwayStatusMessage.verify|verify} messages.
     * @param message SubwayStatusMessage message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: ISubwayStatusMessage, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes a SubwayStatusMessage message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns SubwayStatusMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): SubwayStatusMessage;

    /**
     * Decodes a SubwayStatusMessage message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns SubwayStatusMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): SubwayStatusMessage;

    /**
     * Verifies a SubwayStatusMessage message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates a SubwayStatusMessage message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns SubwayStatusMessage
     */
    public static fromObject(object: { [k: string]: any }): SubwayStatusMessage;

    /**
     * Creates a plain object from a SubwayStatusMessage message. Also converts values to other types if specified.
     * @param message SubwayStatusMessage
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: SubwayStatusMessage, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this SubwayStatusMessage to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}

/** Properties of an AffectedLineStatus. */
export interface IAffectedLineStatus {

    /** AffectedLineStatus line */
    line?: (string|null);

    /** AffectedLineStatus direction */
    direction?: (Direction|null);
}

/** Represents an AffectedLineStatus. */
export class AffectedLineStatus implements IAffectedLineStatus {

    /**
     * Constructs a new AffectedLineStatus.
     * @param [properties] Properties to set
     */
    constructor(properties?: IAffectedLineStatus);

    /** AffectedLineStatus line. */
    public line: string;

    /** AffectedLineStatus direction. */
    public direction: Direction;

    /**
     * Creates a new AffectedLineStatus instance using the specified properties.
     * @param [properties] Properties to set
     * @returns AffectedLineStatus instance
     */
    public static create(properties?: IAffectedLineStatus): AffectedLineStatus;

    /**
     * Encodes the specified AffectedLineStatus message. Does not implicitly {@link AffectedLineStatus.verify|verify} messages.
     * @param message AffectedLineStatus message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encode(message: IAffectedLineStatus, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Encodes the specified AffectedLineStatus message, length delimited. Does not implicitly {@link AffectedLineStatus.verify|verify} messages.
     * @param message AffectedLineStatus message or plain object to encode
     * @param [writer] Writer to encode to
     * @returns Writer
     */
    public static encodeDelimited(message: IAffectedLineStatus, writer?: $protobuf.Writer): $protobuf.Writer;

    /**
     * Decodes an AffectedLineStatus message from the specified reader or buffer.
     * @param reader Reader or buffer to decode from
     * @param [length] Message length if known beforehand
     * @returns AffectedLineStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): AffectedLineStatus;

    /**
     * Decodes an AffectedLineStatus message from the specified reader or buffer, length delimited.
     * @param reader Reader or buffer to decode from
     * @returns AffectedLineStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): AffectedLineStatus;

    /**
     * Verifies an AffectedLineStatus message.
     * @param message Plain object to verify
     * @returns `null` if valid, otherwise the reason why it is not
     */
    public static verify(message: { [k: string]: any }): (string|null);

    /**
     * Creates an AffectedLineStatus message from a plain object. Also converts values to their respective internal types.
     * @param object Plain object
     * @returns AffectedLineStatus
     */
    public static fromObject(object: { [k: string]: any }): AffectedLineStatus;

    /**
     * Creates a plain object from an AffectedLineStatus message. Also converts values to other types if specified.
     * @param message AffectedLineStatus
     * @param [options] Conversion options
     * @returns Plain object
     */
    public static toObject(message: AffectedLineStatus, options?: $protobuf.IConversionOptions): { [k: string]: any };

    /**
     * Converts this AffectedLineStatus to JSON.
     * @returns JSON object
     */
    public toJSON(): { [k: string]: any };
}
