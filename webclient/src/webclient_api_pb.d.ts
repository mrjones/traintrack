import * as $protobuf from "protobufjs";

/** Properties of a LineArrivals. */
export interface ILineArrivals {

    /** LineArrivals line */
    line?: string;

    /** LineArrivals direction */
    direction?: Direction;

    /** LineArrivals timestamp */
    timestamp?: (number|Long)[];
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

    /** LineArrivals timestamp. */
    public timestamp: (number|Long)[];

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

/** Properties of a StationStatus. */
export interface IStationStatus {

    /** StationStatus name */
    name?: string;

    /** StationStatus line */
    line?: ILineArrivals[];
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
// NOTE: THIS WAS ADDED MANUALLY
// https://github.com/dcodeIO/protobuf.js/issues/780
export enum Direction {
    UPTOWN,
    DOWNTOWN,
}
