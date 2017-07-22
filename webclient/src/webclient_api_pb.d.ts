import * as $protobuf from "protobufjs";

/** Properties of a DebugInfo. */
export interface IDebugInfo {

    /** DebugInfo processingTimeMs */
    processingTimeMs?: (number|Long);
}

/** Represents a DebugInfo. */
export class DebugInfo {

    /**
     * Constructs a new DebugInfo.
     * @param [properties] Properties to set
     */
    constructor(properties?: IDebugInfo);

    /** DebugInfo processingTimeMs. */
    public processingTimeMs: (number|Long);

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
    timestamp?: (number|Long);

    /** LineArrival tripId */
    tripId?: string;
}

/** Represents a LineArrival. */
export class LineArrival {

    /**
     * Constructs a new LineArrival.
     * @param [properties] Properties to set
     */
    constructor(properties?: ILineArrival);

    /** LineArrival timestamp. */
    public timestamp: (number|Long);

    /** LineArrival tripId. */
    public tripId: string;

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
    line?: string;

    /** LineArrivals direction */
    direction?: Direction;

    /** LineArrivals lineColorHex */
    lineColorHex?: string;

    /** LineArrivals arrivals */
    arrivals?: ILineArrival[];

    /** LineArrivals debugInfo */
    debugInfo?: IDebugInfo;
}

/** Represents a LineArrivals. */
export class LineArrivals {

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
    timestamp?: (number|Long);

    /** TrainItineraryArrival station */
    station?: IStation;
}

/** Represents a TrainItineraryArrival. */
export class TrainItineraryArrival {

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
    line?: string;

    /** TrainItinerary direction */
    direction?: Direction;

    /** TrainItinerary lineColorHex */
    lineColorHex?: string;

    /** TrainItinerary arrival */
    arrival?: ITrainItineraryArrival[];

    /** TrainItinerary dataTimestamp */
    dataTimestamp?: (number|Long);

    /** TrainItinerary debugInfo */
    debugInfo?: IDebugInfo;
}

/** Represents a TrainItinerary. */
export class TrainItinerary {

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
    name?: string;

    /** StationStatus line */
    line?: ILineArrivals[];

    /** StationStatus dataTimestamp */
    dataTimestamp?: (number|Long);

    /** StationStatus debugInfo */
    debugInfo?: IDebugInfo;
}

/** Represents a StationStatus. */
export class StationStatus {

    /**
     * Constructs a new StationStatus.
     * @param [properties] Properties to set
     */
    constructor(properties?: IStationStatus);

    /** StationStatus name. */
    public name: string;

    /** StationStatus line. */
    public line: ILineArrivals[];

    /** StationStatus dataTimestamp. */
    public dataTimestamp: (number|Long);

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
    id?: string;

    /** Station name */
    name?: string;
}

/** Represents a Station. */
export class Station {

    /**
     * Constructs a new Station.
     * @param [properties] Properties to set
     */
    constructor(properties?: IStation);

    /** Station id. */
    public id: string;

    /** Station name. */
    public name: string;

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
    station?: IStation[];

    /** StationList debugInfo */
    debugInfo?: IDebugInfo;
}

/** Represents a StationList. */
export class StationList {

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
    name?: string;

    /** Line colorHex */
    colorHex?: string;

    /** Line active */
    active?: boolean;
}

/** Represents a Line. */
export class Line {

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
    line?: ILine[];

    /** LineList debugInfo */
    debugInfo?: IDebugInfo;
}

/** Represents a LineList. */
export class LineList {

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
// NOTE: THIS WAS ADDED MANUALLY
// https://github.com/dcodeIO/protobuf.js/issues/780
export enum Direction {
    UPTOWN,
    DOWNTOWN,
}
