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
/** Namespace webclient_api. */
export namespace webclient_api {

    /** Direction enum. */
    enum Direction {
        UPTOWN = 0,
        DOWNTOWN = 1
    }

    /** Properties of a DebugInfo. */
    interface IDebugInfo {

        /** DebugInfo processingTimeMs */
        processingTimeMs?: (number|Long|null);

        /** DebugInfo buildVersion */
        buildVersion?: (string|null);

        /** DebugInfo buildTimestamp */
        buildTimestamp?: (number|Long|null);
    }

    /** Represents a DebugInfo. */
    class DebugInfo implements IDebugInfo {

        /**
         * Constructs a new DebugInfo.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.IDebugInfo);

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
        public static create(properties?: webclient_api.IDebugInfo): webclient_api.DebugInfo;

        /**
         * Encodes the specified DebugInfo message. Does not implicitly {@link webclient_api.DebugInfo.verify|verify} messages.
         * @param message DebugInfo message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.IDebugInfo, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified DebugInfo message, length delimited. Does not implicitly {@link webclient_api.DebugInfo.verify|verify} messages.
         * @param message DebugInfo message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.IDebugInfo, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a DebugInfo message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns DebugInfo
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.DebugInfo;

        /**
         * Decodes a DebugInfo message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns DebugInfo
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.DebugInfo;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.DebugInfo;

        /**
         * Creates a plain object from a DebugInfo message. Also converts values to other types if specified.
         * @param message DebugInfo
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.DebugInfo, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this DebugInfo to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a LineArrival. */
    interface ILineArrival {

        /** LineArrival timestamp */
        timestamp?: (number|Long|null);

        /** LineArrival tripId */
        tripId?: (string|null);

        /** LineArrival headsign */
        headsign?: (string|null);
    }

    /** Represents a LineArrival. */
    class LineArrival implements ILineArrival {

        /**
         * Constructs a new LineArrival.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ILineArrival);

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
        public static create(properties?: webclient_api.ILineArrival): webclient_api.LineArrival;

        /**
         * Encodes the specified LineArrival message. Does not implicitly {@link webclient_api.LineArrival.verify|verify} messages.
         * @param message LineArrival message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ILineArrival, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified LineArrival message, length delimited. Does not implicitly {@link webclient_api.LineArrival.verify|verify} messages.
         * @param message LineArrival message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ILineArrival, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a LineArrival message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns LineArrival
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.LineArrival;

        /**
         * Decodes a LineArrival message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns LineArrival
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.LineArrival;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.LineArrival;

        /**
         * Creates a plain object from a LineArrival message. Also converts values to other types if specified.
         * @param message LineArrival
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.LineArrival, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this LineArrival to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a LineArrivals. */
    interface ILineArrivals {

        /** LineArrivals line */
        line?: (string|null);

        /** LineArrivals direction */
        direction?: (webclient_api.Direction|null);

        /** LineArrivals lineColorHex */
        lineColorHex?: (string|null);

        /** LineArrivals arrivals */
        arrivals?: (webclient_api.ILineArrival[]|null);

        /** LineArrivals debugInfo */
        debugInfo?: (webclient_api.IDebugInfo|null);
    }

    /** Represents a LineArrivals. */
    class LineArrivals implements ILineArrivals {

        /**
         * Constructs a new LineArrivals.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ILineArrivals);

        /** LineArrivals line. */
        public line: string;

        /** LineArrivals direction. */
        public direction: webclient_api.Direction;

        /** LineArrivals lineColorHex. */
        public lineColorHex: string;

        /** LineArrivals arrivals. */
        public arrivals: webclient_api.ILineArrival[];

        /** LineArrivals debugInfo. */
        public debugInfo?: (webclient_api.IDebugInfo|null);

        /**
         * Creates a new LineArrivals instance using the specified properties.
         * @param [properties] Properties to set
         * @returns LineArrivals instance
         */
        public static create(properties?: webclient_api.ILineArrivals): webclient_api.LineArrivals;

        /**
         * Encodes the specified LineArrivals message. Does not implicitly {@link webclient_api.LineArrivals.verify|verify} messages.
         * @param message LineArrivals message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ILineArrivals, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified LineArrivals message, length delimited. Does not implicitly {@link webclient_api.LineArrivals.verify|verify} messages.
         * @param message LineArrivals message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ILineArrivals, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a LineArrivals message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns LineArrivals
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.LineArrivals;

        /**
         * Decodes a LineArrivals message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns LineArrivals
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.LineArrivals;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.LineArrivals;

        /**
         * Creates a plain object from a LineArrivals message. Also converts values to other types if specified.
         * @param message LineArrivals
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.LineArrivals, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this LineArrivals to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TrainItineraryArrival. */
    interface ITrainItineraryArrival {

        /** TrainItineraryArrival timestamp */
        timestamp?: (number|Long|null);

        /** TrainItineraryArrival station */
        station?: (webclient_api.IStation|null);
    }

    /** Represents a TrainItineraryArrival. */
    class TrainItineraryArrival implements ITrainItineraryArrival {

        /**
         * Constructs a new TrainItineraryArrival.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ITrainItineraryArrival);

        /** TrainItineraryArrival timestamp. */
        public timestamp: (number|Long);

        /** TrainItineraryArrival station. */
        public station?: (webclient_api.IStation|null);

        /**
         * Creates a new TrainItineraryArrival instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TrainItineraryArrival instance
         */
        public static create(properties?: webclient_api.ITrainItineraryArrival): webclient_api.TrainItineraryArrival;

        /**
         * Encodes the specified TrainItineraryArrival message. Does not implicitly {@link webclient_api.TrainItineraryArrival.verify|verify} messages.
         * @param message TrainItineraryArrival message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ITrainItineraryArrival, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TrainItineraryArrival message, length delimited. Does not implicitly {@link webclient_api.TrainItineraryArrival.verify|verify} messages.
         * @param message TrainItineraryArrival message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ITrainItineraryArrival, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TrainItineraryArrival message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TrainItineraryArrival
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.TrainItineraryArrival;

        /**
         * Decodes a TrainItineraryArrival message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TrainItineraryArrival
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.TrainItineraryArrival;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.TrainItineraryArrival;

        /**
         * Creates a plain object from a TrainItineraryArrival message. Also converts values to other types if specified.
         * @param message TrainItineraryArrival
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.TrainItineraryArrival, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TrainItineraryArrival to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TrainItinerary. */
    interface ITrainItinerary {

        /** TrainItinerary line */
        line?: (string|null);

        /** TrainItinerary direction */
        direction?: (webclient_api.Direction|null);

        /** TrainItinerary lineColorHex */
        lineColorHex?: (string|null);

        /** TrainItinerary arrival */
        arrival?: (webclient_api.ITrainItineraryArrival[]|null);

        /** TrainItinerary dataTimestamp */
        dataTimestamp?: (number|Long|null);

        /** TrainItinerary debugInfo */
        debugInfo?: (webclient_api.IDebugInfo|null);
    }

    /** Represents a TrainItinerary. */
    class TrainItinerary implements ITrainItinerary {

        /**
         * Constructs a new TrainItinerary.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ITrainItinerary);

        /** TrainItinerary line. */
        public line: string;

        /** TrainItinerary direction. */
        public direction: webclient_api.Direction;

        /** TrainItinerary lineColorHex. */
        public lineColorHex: string;

        /** TrainItinerary arrival. */
        public arrival: webclient_api.ITrainItineraryArrival[];

        /** TrainItinerary dataTimestamp. */
        public dataTimestamp: (number|Long);

        /** TrainItinerary debugInfo. */
        public debugInfo?: (webclient_api.IDebugInfo|null);

        /**
         * Creates a new TrainItinerary instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TrainItinerary instance
         */
        public static create(properties?: webclient_api.ITrainItinerary): webclient_api.TrainItinerary;

        /**
         * Encodes the specified TrainItinerary message. Does not implicitly {@link webclient_api.TrainItinerary.verify|verify} messages.
         * @param message TrainItinerary message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ITrainItinerary, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TrainItinerary message, length delimited. Does not implicitly {@link webclient_api.TrainItinerary.verify|verify} messages.
         * @param message TrainItinerary message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ITrainItinerary, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TrainItinerary message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TrainItinerary
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.TrainItinerary;

        /**
         * Decodes a TrainItinerary message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TrainItinerary
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.TrainItinerary;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.TrainItinerary;

        /**
         * Creates a plain object from a TrainItinerary message. Also converts values to other types if specified.
         * @param message TrainItinerary
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.TrainItinerary, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TrainItinerary to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a StationStatus. */
    interface IStationStatus {

        /** StationStatus name */
        name?: (string|null);

        /** StationStatus id */
        id?: (string|null);

        /** StationStatus line */
        line?: (webclient_api.ILineArrivals[]|null);

        /** StationStatus dataTimestamp */
        dataTimestamp?: (number|Long|null);

        /** StationStatus statusMessage */
        statusMessage?: (webclient_api.ISubwayStatusMessage[]|null);

        /** StationStatus debugInfo */
        debugInfo?: (webclient_api.IDebugInfo|null);
    }

    /** Represents a StationStatus. */
    class StationStatus implements IStationStatus {

        /**
         * Constructs a new StationStatus.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.IStationStatus);

        /** StationStatus name. */
        public name: string;

        /** StationStatus id. */
        public id: string;

        /** StationStatus line. */
        public line: webclient_api.ILineArrivals[];

        /** StationStatus dataTimestamp. */
        public dataTimestamp: (number|Long);

        /** StationStatus statusMessage. */
        public statusMessage: webclient_api.ISubwayStatusMessage[];

        /** StationStatus debugInfo. */
        public debugInfo?: (webclient_api.IDebugInfo|null);

        /**
         * Creates a new StationStatus instance using the specified properties.
         * @param [properties] Properties to set
         * @returns StationStatus instance
         */
        public static create(properties?: webclient_api.IStationStatus): webclient_api.StationStatus;

        /**
         * Encodes the specified StationStatus message. Does not implicitly {@link webclient_api.StationStatus.verify|verify} messages.
         * @param message StationStatus message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.IStationStatus, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified StationStatus message, length delimited. Does not implicitly {@link webclient_api.StationStatus.verify|verify} messages.
         * @param message StationStatus message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.IStationStatus, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a StationStatus message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns StationStatus
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.StationStatus;

        /**
         * Decodes a StationStatus message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns StationStatus
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.StationStatus;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.StationStatus;

        /**
         * Creates a plain object from a StationStatus message. Also converts values to other types if specified.
         * @param message StationStatus
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.StationStatus, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this StationStatus to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a Station. */
    interface IStation {

        /** Station id */
        id?: (string|null);

        /** Station name */
        name?: (string|null);

        /** Station lines */
        lines?: (string[]|null);
    }

    /** Represents a Station. */
    class Station implements IStation {

        /**
         * Constructs a new Station.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.IStation);

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
        public static create(properties?: webclient_api.IStation): webclient_api.Station;

        /**
         * Encodes the specified Station message. Does not implicitly {@link webclient_api.Station.verify|verify} messages.
         * @param message Station message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.IStation, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified Station message, length delimited. Does not implicitly {@link webclient_api.Station.verify|verify} messages.
         * @param message Station message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.IStation, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a Station message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns Station
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.Station;

        /**
         * Decodes a Station message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns Station
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.Station;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.Station;

        /**
         * Creates a plain object from a Station message. Also converts values to other types if specified.
         * @param message Station
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.Station, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this Station to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a StationList. */
    interface IStationList {

        /** StationList station */
        station?: (webclient_api.IStation[]|null);

        /** StationList debugInfo */
        debugInfo?: (webclient_api.IDebugInfo|null);
    }

    /** Represents a StationList. */
    class StationList implements IStationList {

        /**
         * Constructs a new StationList.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.IStationList);

        /** StationList station. */
        public station: webclient_api.IStation[];

        /** StationList debugInfo. */
        public debugInfo?: (webclient_api.IDebugInfo|null);

        /**
         * Creates a new StationList instance using the specified properties.
         * @param [properties] Properties to set
         * @returns StationList instance
         */
        public static create(properties?: webclient_api.IStationList): webclient_api.StationList;

        /**
         * Encodes the specified StationList message. Does not implicitly {@link webclient_api.StationList.verify|verify} messages.
         * @param message StationList message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.IStationList, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified StationList message, length delimited. Does not implicitly {@link webclient_api.StationList.verify|verify} messages.
         * @param message StationList message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.IStationList, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a StationList message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns StationList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.StationList;

        /**
         * Decodes a StationList message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns StationList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.StationList;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.StationList;

        /**
         * Creates a plain object from a StationList message. Also converts values to other types if specified.
         * @param message StationList
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.StationList, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this StationList to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a Line. */
    interface ILine {

        /** Line name */
        name?: (string|null);

        /** Line colorHex */
        colorHex?: (string|null);

        /** Line active */
        active?: (boolean|null);
    }

    /** Represents a Line. */
    class Line implements ILine {

        /**
         * Constructs a new Line.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ILine);

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
        public static create(properties?: webclient_api.ILine): webclient_api.Line;

        /**
         * Encodes the specified Line message. Does not implicitly {@link webclient_api.Line.verify|verify} messages.
         * @param message Line message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ILine, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified Line message, length delimited. Does not implicitly {@link webclient_api.Line.verify|verify} messages.
         * @param message Line message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ILine, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a Line message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns Line
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.Line;

        /**
         * Decodes a Line message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns Line
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.Line;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.Line;

        /**
         * Creates a plain object from a Line message. Also converts values to other types if specified.
         * @param message Line
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.Line, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this Line to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a LineList. */
    interface ILineList {

        /** LineList line */
        line?: (webclient_api.ILine[]|null);

        /** LineList debugInfo */
        debugInfo?: (webclient_api.IDebugInfo|null);
    }

    /** Represents a LineList. */
    class LineList implements ILineList {

        /**
         * Constructs a new LineList.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ILineList);

        /** LineList line. */
        public line: webclient_api.ILine[];

        /** LineList debugInfo. */
        public debugInfo?: (webclient_api.IDebugInfo|null);

        /**
         * Creates a new LineList instance using the specified properties.
         * @param [properties] Properties to set
         * @returns LineList instance
         */
        public static create(properties?: webclient_api.ILineList): webclient_api.LineList;

        /**
         * Encodes the specified LineList message. Does not implicitly {@link webclient_api.LineList.verify|verify} messages.
         * @param message LineList message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ILineList, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified LineList message, length delimited. Does not implicitly {@link webclient_api.LineList.verify|verify} messages.
         * @param message LineList message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ILineList, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a LineList message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns LineList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.LineList;

        /**
         * Decodes a LineList message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns LineList
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.LineList;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.LineList;

        /**
         * Creates a plain object from a LineList message. Also converts values to other types if specified.
         * @param message LineList
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.LineList, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this LineList to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TrainArrivalHistoryEntry. */
    interface ITrainArrivalHistoryEntry {

        /** TrainArrivalHistoryEntry dataTimestamp */
        dataTimestamp?: (number|Long|null);

        /** TrainArrivalHistoryEntry arrivalTime */
        arrivalTime?: (number|Long|null);
    }

    /** Represents a TrainArrivalHistoryEntry. */
    class TrainArrivalHistoryEntry implements ITrainArrivalHistoryEntry {

        /**
         * Constructs a new TrainArrivalHistoryEntry.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ITrainArrivalHistoryEntry);

        /** TrainArrivalHistoryEntry dataTimestamp. */
        public dataTimestamp: (number|Long);

        /** TrainArrivalHistoryEntry arrivalTime. */
        public arrivalTime: (number|Long);

        /**
         * Creates a new TrainArrivalHistoryEntry instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TrainArrivalHistoryEntry instance
         */
        public static create(properties?: webclient_api.ITrainArrivalHistoryEntry): webclient_api.TrainArrivalHistoryEntry;

        /**
         * Encodes the specified TrainArrivalHistoryEntry message. Does not implicitly {@link webclient_api.TrainArrivalHistoryEntry.verify|verify} messages.
         * @param message TrainArrivalHistoryEntry message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ITrainArrivalHistoryEntry, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TrainArrivalHistoryEntry message, length delimited. Does not implicitly {@link webclient_api.TrainArrivalHistoryEntry.verify|verify} messages.
         * @param message TrainArrivalHistoryEntry message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ITrainArrivalHistoryEntry, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TrainArrivalHistoryEntry message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TrainArrivalHistoryEntry
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.TrainArrivalHistoryEntry;

        /**
         * Decodes a TrainArrivalHistoryEntry message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TrainArrivalHistoryEntry
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.TrainArrivalHistoryEntry;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.TrainArrivalHistoryEntry;

        /**
         * Creates a plain object from a TrainArrivalHistoryEntry message. Also converts values to other types if specified.
         * @param message TrainArrivalHistoryEntry
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.TrainArrivalHistoryEntry, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TrainArrivalHistoryEntry to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a TrainArrivalHistory. */
    interface ITrainArrivalHistory {

        /** TrainArrivalHistory debugInfo */
        debugInfo?: (webclient_api.IDebugInfo|null);

        /** TrainArrivalHistory history */
        history?: (webclient_api.ITrainArrivalHistoryEntry[]|null);
    }

    /** Represents a TrainArrivalHistory. */
    class TrainArrivalHistory implements ITrainArrivalHistory {

        /**
         * Constructs a new TrainArrivalHistory.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ITrainArrivalHistory);

        /** TrainArrivalHistory debugInfo. */
        public debugInfo?: (webclient_api.IDebugInfo|null);

        /** TrainArrivalHistory history. */
        public history: webclient_api.ITrainArrivalHistoryEntry[];

        /**
         * Creates a new TrainArrivalHistory instance using the specified properties.
         * @param [properties] Properties to set
         * @returns TrainArrivalHistory instance
         */
        public static create(properties?: webclient_api.ITrainArrivalHistory): webclient_api.TrainArrivalHistory;

        /**
         * Encodes the specified TrainArrivalHistory message. Does not implicitly {@link webclient_api.TrainArrivalHistory.verify|verify} messages.
         * @param message TrainArrivalHistory message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ITrainArrivalHistory, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified TrainArrivalHistory message, length delimited. Does not implicitly {@link webclient_api.TrainArrivalHistory.verify|verify} messages.
         * @param message TrainArrivalHistory message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ITrainArrivalHistory, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a TrainArrivalHistory message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns TrainArrivalHistory
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.TrainArrivalHistory;

        /**
         * Decodes a TrainArrivalHistory message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns TrainArrivalHistory
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.TrainArrivalHistory;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.TrainArrivalHistory;

        /**
         * Creates a plain object from a TrainArrivalHistory message. Also converts values to other types if specified.
         * @param message TrainArrivalHistory
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.TrainArrivalHistory, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this TrainArrivalHistory to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of a SubwayStatusMessage. */
    interface ISubwayStatusMessage {

        /** SubwayStatusMessage summary */
        summary?: (string|null);

        /** SubwayStatusMessage longDescription */
        longDescription?: (string|null);

        /** SubwayStatusMessage affectedLine */
        affectedLine?: (webclient_api.IAffectedLineStatus[]|null);

        /** SubwayStatusMessage planned */
        planned?: (boolean|null);

        /** SubwayStatusMessage reasonName */
        reasonName?: (string|null);

        /** SubwayStatusMessage priority */
        priority?: (number|null);

        /** SubwayStatusMessage publishTimestamp */
        publishTimestamp?: (number|Long|null);

        /** SubwayStatusMessage id */
        id?: (string|null);
    }

    /** Represents a SubwayStatusMessage. */
    class SubwayStatusMessage implements ISubwayStatusMessage {

        /**
         * Constructs a new SubwayStatusMessage.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.ISubwayStatusMessage);

        /** SubwayStatusMessage summary. */
        public summary: string;

        /** SubwayStatusMessage longDescription. */
        public longDescription: string;

        /** SubwayStatusMessage affectedLine. */
        public affectedLine: webclient_api.IAffectedLineStatus[];

        /** SubwayStatusMessage planned. */
        public planned: boolean;

        /** SubwayStatusMessage reasonName. */
        public reasonName: string;

        /** SubwayStatusMessage priority. */
        public priority: number;

        /** SubwayStatusMessage publishTimestamp. */
        public publishTimestamp: (number|Long);

        /** SubwayStatusMessage id. */
        public id: string;

        /**
         * Creates a new SubwayStatusMessage instance using the specified properties.
         * @param [properties] Properties to set
         * @returns SubwayStatusMessage instance
         */
        public static create(properties?: webclient_api.ISubwayStatusMessage): webclient_api.SubwayStatusMessage;

        /**
         * Encodes the specified SubwayStatusMessage message. Does not implicitly {@link webclient_api.SubwayStatusMessage.verify|verify} messages.
         * @param message SubwayStatusMessage message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.ISubwayStatusMessage, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified SubwayStatusMessage message, length delimited. Does not implicitly {@link webclient_api.SubwayStatusMessage.verify|verify} messages.
         * @param message SubwayStatusMessage message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.ISubwayStatusMessage, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes a SubwayStatusMessage message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns SubwayStatusMessage
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.SubwayStatusMessage;

        /**
         * Decodes a SubwayStatusMessage message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns SubwayStatusMessage
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.SubwayStatusMessage;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.SubwayStatusMessage;

        /**
         * Creates a plain object from a SubwayStatusMessage message. Also converts values to other types if specified.
         * @param message SubwayStatusMessage
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.SubwayStatusMessage, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this SubwayStatusMessage to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }

    /** Properties of an AffectedLineStatus. */
    interface IAffectedLineStatus {

        /** AffectedLineStatus line */
        line?: (string|null);

        /** AffectedLineStatus direction */
        direction?: (webclient_api.Direction|null);
    }

    /** Represents an AffectedLineStatus. */
    class AffectedLineStatus implements IAffectedLineStatus {

        /**
         * Constructs a new AffectedLineStatus.
         * @param [properties] Properties to set
         */
        constructor(properties?: webclient_api.IAffectedLineStatus);

        /** AffectedLineStatus line. */
        public line: string;

        /** AffectedLineStatus direction. */
        public direction: webclient_api.Direction;

        /**
         * Creates a new AffectedLineStatus instance using the specified properties.
         * @param [properties] Properties to set
         * @returns AffectedLineStatus instance
         */
        public static create(properties?: webclient_api.IAffectedLineStatus): webclient_api.AffectedLineStatus;

        /**
         * Encodes the specified AffectedLineStatus message. Does not implicitly {@link webclient_api.AffectedLineStatus.verify|verify} messages.
         * @param message AffectedLineStatus message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encode(message: webclient_api.IAffectedLineStatus, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Encodes the specified AffectedLineStatus message, length delimited. Does not implicitly {@link webclient_api.AffectedLineStatus.verify|verify} messages.
         * @param message AffectedLineStatus message or plain object to encode
         * @param [writer] Writer to encode to
         * @returns Writer
         */
        public static encodeDelimited(message: webclient_api.IAffectedLineStatus, writer?: $protobuf.Writer): $protobuf.Writer;

        /**
         * Decodes an AffectedLineStatus message from the specified reader or buffer.
         * @param reader Reader or buffer to decode from
         * @param [length] Message length if known beforehand
         * @returns AffectedLineStatus
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): webclient_api.AffectedLineStatus;

        /**
         * Decodes an AffectedLineStatus message from the specified reader or buffer, length delimited.
         * @param reader Reader or buffer to decode from
         * @returns AffectedLineStatus
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): webclient_api.AffectedLineStatus;

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
        public static fromObject(object: { [k: string]: any }): webclient_api.AffectedLineStatus;

        /**
         * Creates a plain object from an AffectedLineStatus message. Also converts values to other types if specified.
         * @param message AffectedLineStatus
         * @param [options] Conversion options
         * @returns Plain object
         */
        public static toObject(message: webclient_api.AffectedLineStatus, options?: $protobuf.IConversionOptions): { [k: string]: any };

        /**
         * Converts this AffectedLineStatus to JSON.
         * @returns JSON object
         */
        public toJSON(): { [k: string]: any };
    }
}
