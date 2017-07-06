/*eslint-disable block-scoped-var, no-redeclare, no-control-regex, no-prototype-builtins*/
"use strict";

var $protobuf = require("protobufjs/minimal");

// Common aliases
var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

/**
 * Direction enum.
 * @exports Direction
 * @enum {string}
 * @property {number} UPTOWN=0 UPTOWN value
 * @property {number} DOWNTOWN=1 DOWNTOWN value
 */
$root.Direction = (function() {
    var valuesById = {}, values = Object.create(valuesById);
    values[valuesById[0] = "UPTOWN"] = 0;
    values[valuesById[1] = "DOWNTOWN"] = 1;
    return values;
})();

$root.DebugInfo = (function() {

    /**
     * Properties of a DebugInfo.
     * @exports IDebugInfo
     * @interface IDebugInfo
     * @property {number|Long} [processingTimeMs] DebugInfo processingTimeMs
     */

    /**
     * Constructs a new DebugInfo.
     * @exports DebugInfo
     * @classdesc Represents a DebugInfo.
     * @constructor
     * @param {IDebugInfo=} [properties] Properties to set
     */
    function DebugInfo(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * DebugInfo processingTimeMs.
     * @member {number|Long}processingTimeMs
     * @memberof DebugInfo
     * @instance
     */
    DebugInfo.prototype.processingTimeMs = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * Creates a new DebugInfo instance using the specified properties.
     * @function create
     * @memberof DebugInfo
     * @static
     * @param {IDebugInfo=} [properties] Properties to set
     * @returns {DebugInfo} DebugInfo instance
     */
    DebugInfo.create = function create(properties) {
        return new DebugInfo(properties);
    };

    /**
     * Encodes the specified DebugInfo message. Does not implicitly {@link DebugInfo.verify|verify} messages.
     * @function encode
     * @memberof DebugInfo
     * @static
     * @param {IDebugInfo} message DebugInfo message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    DebugInfo.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.processingTimeMs != null && message.hasOwnProperty("processingTimeMs"))
            writer.uint32(/* id 1, wireType 0 =*/8).int64(message.processingTimeMs);
        return writer;
    };

    /**
     * Encodes the specified DebugInfo message, length delimited. Does not implicitly {@link DebugInfo.verify|verify} messages.
     * @function encodeDelimited
     * @memberof DebugInfo
     * @static
     * @param {IDebugInfo} message DebugInfo message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    DebugInfo.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a DebugInfo message from the specified reader or buffer.
     * @function decode
     * @memberof DebugInfo
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {DebugInfo} DebugInfo
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    DebugInfo.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.DebugInfo();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.processingTimeMs = reader.int64();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a DebugInfo message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof DebugInfo
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {DebugInfo} DebugInfo
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    DebugInfo.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a DebugInfo message.
     * @function verify
     * @memberof DebugInfo
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    DebugInfo.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.processingTimeMs != null && message.hasOwnProperty("processingTimeMs"))
            if (!$util.isInteger(message.processingTimeMs) && !(message.processingTimeMs && $util.isInteger(message.processingTimeMs.low) && $util.isInteger(message.processingTimeMs.high)))
                return "processingTimeMs: integer|Long expected";
        return null;
    };

    /**
     * Creates a DebugInfo message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof DebugInfo
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {DebugInfo} DebugInfo
     */
    DebugInfo.fromObject = function fromObject(object) {
        if (object instanceof $root.DebugInfo)
            return object;
        var message = new $root.DebugInfo();
        if (object.processingTimeMs != null)
            if ($util.Long)
                (message.processingTimeMs = $util.Long.fromValue(object.processingTimeMs)).unsigned = false;
            else if (typeof object.processingTimeMs === "string")
                message.processingTimeMs = parseInt(object.processingTimeMs, 10);
            else if (typeof object.processingTimeMs === "number")
                message.processingTimeMs = object.processingTimeMs;
            else if (typeof object.processingTimeMs === "object")
                message.processingTimeMs = new $util.LongBits(object.processingTimeMs.low >>> 0, object.processingTimeMs.high >>> 0).toNumber();
        return message;
    };

    /**
     * Creates a plain object from a DebugInfo message. Also converts values to other types if specified.
     * @function toObject
     * @memberof DebugInfo
     * @static
     * @param {DebugInfo} message DebugInfo
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    DebugInfo.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults)
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.processingTimeMs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.processingTimeMs = options.longs === String ? "0" : 0;
        if (message.processingTimeMs != null && message.hasOwnProperty("processingTimeMs"))
            if (typeof message.processingTimeMs === "number")
                object.processingTimeMs = options.longs === String ? String(message.processingTimeMs) : message.processingTimeMs;
            else
                object.processingTimeMs = options.longs === String ? $util.Long.prototype.toString.call(message.processingTimeMs) : options.longs === Number ? new $util.LongBits(message.processingTimeMs.low >>> 0, message.processingTimeMs.high >>> 0).toNumber() : message.processingTimeMs;
        return object;
    };

    /**
     * Converts this DebugInfo to JSON.
     * @function toJSON
     * @memberof DebugInfo
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    DebugInfo.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return DebugInfo;
})();

$root.Arrival = (function() {

    /**
     * Properties of an Arrival.
     * @exports IArrival
     * @interface IArrival
     * @property {number|Long} [timestamp] Arrival timestamp
     * @property {string} [tripId] Arrival tripId
     */

    /**
     * Constructs a new Arrival.
     * @exports Arrival
     * @classdesc Represents an Arrival.
     * @constructor
     * @param {IArrival=} [properties] Properties to set
     */
    function Arrival(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * Arrival timestamp.
     * @member {number|Long}timestamp
     * @memberof Arrival
     * @instance
     */
    Arrival.prototype.timestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * Arrival tripId.
     * @member {string}tripId
     * @memberof Arrival
     * @instance
     */
    Arrival.prototype.tripId = "";

    /**
     * Creates a new Arrival instance using the specified properties.
     * @function create
     * @memberof Arrival
     * @static
     * @param {IArrival=} [properties] Properties to set
     * @returns {Arrival} Arrival instance
     */
    Arrival.create = function create(properties) {
        return new Arrival(properties);
    };

    /**
     * Encodes the specified Arrival message. Does not implicitly {@link Arrival.verify|verify} messages.
     * @function encode
     * @memberof Arrival
     * @static
     * @param {IArrival} message Arrival message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Arrival.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            writer.uint32(/* id 1, wireType 0 =*/8).int64(message.timestamp);
        if (message.tripId != null && message.hasOwnProperty("tripId"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.tripId);
        return writer;
    };

    /**
     * Encodes the specified Arrival message, length delimited. Does not implicitly {@link Arrival.verify|verify} messages.
     * @function encodeDelimited
     * @memberof Arrival
     * @static
     * @param {IArrival} message Arrival message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Arrival.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes an Arrival message from the specified reader or buffer.
     * @function decode
     * @memberof Arrival
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {Arrival} Arrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Arrival.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.Arrival();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.timestamp = reader.int64();
                break;
            case 2:
                message.tripId = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes an Arrival message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof Arrival
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {Arrival} Arrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Arrival.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies an Arrival message.
     * @function verify
     * @memberof Arrival
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    Arrival.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            if (!$util.isInteger(message.timestamp) && !(message.timestamp && $util.isInteger(message.timestamp.low) && $util.isInteger(message.timestamp.high)))
                return "timestamp: integer|Long expected";
        if (message.tripId != null && message.hasOwnProperty("tripId"))
            if (!$util.isString(message.tripId))
                return "tripId: string expected";
        return null;
    };

    /**
     * Creates an Arrival message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof Arrival
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {Arrival} Arrival
     */
    Arrival.fromObject = function fromObject(object) {
        if (object instanceof $root.Arrival)
            return object;
        var message = new $root.Arrival();
        if (object.timestamp != null)
            if ($util.Long)
                (message.timestamp = $util.Long.fromValue(object.timestamp)).unsigned = false;
            else if (typeof object.timestamp === "string")
                message.timestamp = parseInt(object.timestamp, 10);
            else if (typeof object.timestamp === "number")
                message.timestamp = object.timestamp;
            else if (typeof object.timestamp === "object")
                message.timestamp = new $util.LongBits(object.timestamp.low >>> 0, object.timestamp.high >>> 0).toNumber();
        if (object.tripId != null)
            message.tripId = String(object.tripId);
        return message;
    };

    /**
     * Creates a plain object from an Arrival message. Also converts values to other types if specified.
     * @function toObject
     * @memberof Arrival
     * @static
     * @param {Arrival} message Arrival
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    Arrival.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.timestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.timestamp = options.longs === String ? "0" : 0;
            object.tripId = "";
        }
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            if (typeof message.timestamp === "number")
                object.timestamp = options.longs === String ? String(message.timestamp) : message.timestamp;
            else
                object.timestamp = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp) : options.longs === Number ? new $util.LongBits(message.timestamp.low >>> 0, message.timestamp.high >>> 0).toNumber() : message.timestamp;
        if (message.tripId != null && message.hasOwnProperty("tripId"))
            object.tripId = message.tripId;
        return object;
    };

    /**
     * Converts this Arrival to JSON.
     * @function toJSON
     * @memberof Arrival
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    Arrival.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return Arrival;
})();

$root.LineArrivals = (function() {

    /**
     * Properties of a LineArrivals.
     * @exports ILineArrivals
     * @interface ILineArrivals
     * @property {string} [line] LineArrivals line
     * @property {Direction} [direction] LineArrivals direction
     * @property {string} [lineColorHex] LineArrivals lineColorHex
     * @property {Array.<IArrival>} [arrivals] LineArrivals arrivals
     * @property {IDebugInfo} [debugInfo] LineArrivals debugInfo
     */

    /**
     * Constructs a new LineArrivals.
     * @exports LineArrivals
     * @classdesc Represents a LineArrivals.
     * @constructor
     * @param {ILineArrivals=} [properties] Properties to set
     */
    function LineArrivals(properties) {
        this.arrivals = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * LineArrivals line.
     * @member {string}line
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.line = "";

    /**
     * LineArrivals direction.
     * @member {Direction}direction
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.direction = 0;

    /**
     * LineArrivals lineColorHex.
     * @member {string}lineColorHex
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.lineColorHex = "";

    /**
     * LineArrivals arrivals.
     * @member {Array.<IArrival>}arrivals
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.arrivals = $util.emptyArray;

    /**
     * LineArrivals debugInfo.
     * @member {(IDebugInfo|null|undefined)}debugInfo
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.debugInfo = null;

    /**
     * Creates a new LineArrivals instance using the specified properties.
     * @function create
     * @memberof LineArrivals
     * @static
     * @param {ILineArrivals=} [properties] Properties to set
     * @returns {LineArrivals} LineArrivals instance
     */
    LineArrivals.create = function create(properties) {
        return new LineArrivals(properties);
    };

    /**
     * Encodes the specified LineArrivals message. Does not implicitly {@link LineArrivals.verify|verify} messages.
     * @function encode
     * @memberof LineArrivals
     * @static
     * @param {ILineArrivals} message LineArrivals message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineArrivals.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.line != null && message.hasOwnProperty("line"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.line);
        if (message.direction != null && message.hasOwnProperty("direction"))
            writer.uint32(/* id 2, wireType 0 =*/16).int32(message.direction);
        if (message.lineColorHex != null && message.hasOwnProperty("lineColorHex"))
            writer.uint32(/* id 4, wireType 2 =*/34).string(message.lineColorHex);
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            $root.DebugInfo.encode(message.debugInfo, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
        if (message.arrivals != null && message.arrivals.length)
            for (var i = 0; i < message.arrivals.length; ++i)
                $root.Arrival.encode(message.arrivals[i], writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified LineArrivals message, length delimited. Does not implicitly {@link LineArrivals.verify|verify} messages.
     * @function encodeDelimited
     * @memberof LineArrivals
     * @static
     * @param {ILineArrivals} message LineArrivals message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineArrivals.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a LineArrivals message from the specified reader or buffer.
     * @function decode
     * @memberof LineArrivals
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {LineArrivals} LineArrivals
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineArrivals.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.LineArrivals();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.line = reader.string();
                break;
            case 2:
                message.direction = reader.int32();
                break;
            case 4:
                message.lineColorHex = reader.string();
                break;
            case 6:
                if (!(message.arrivals && message.arrivals.length))
                    message.arrivals = [];
                message.arrivals.push($root.Arrival.decode(reader, reader.uint32()));
                break;
            case 5:
                message.debugInfo = $root.DebugInfo.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a LineArrivals message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof LineArrivals
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {LineArrivals} LineArrivals
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineArrivals.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a LineArrivals message.
     * @function verify
     * @memberof LineArrivals
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    LineArrivals.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.line != null && message.hasOwnProperty("line"))
            if (!$util.isString(message.line))
                return "line: string expected";
        if (message.direction != null && message.hasOwnProperty("direction"))
            switch (message.direction) {
            default:
                return "direction: enum value expected";
            case 0:
            case 1:
                break;
            }
        if (message.lineColorHex != null && message.hasOwnProperty("lineColorHex"))
            if (!$util.isString(message.lineColorHex))
                return "lineColorHex: string expected";
        if (message.arrivals != null && message.hasOwnProperty("arrivals")) {
            if (!Array.isArray(message.arrivals))
                return "arrivals: array expected";
            for (var i = 0; i < message.arrivals.length; ++i) {
                var error = $root.Arrival.verify(message.arrivals[i]);
                if (error)
                    return "arrivals." + error;
            }
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            error = $root.DebugInfo.verify(message.debugInfo);
            if (error)
                return "debugInfo." + error;
        }
        return null;
    };

    /**
     * Creates a LineArrivals message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof LineArrivals
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {LineArrivals} LineArrivals
     */
    LineArrivals.fromObject = function fromObject(object) {
        if (object instanceof $root.LineArrivals)
            return object;
        var message = new $root.LineArrivals();
        if (object.line != null)
            message.line = String(object.line);
        switch (object.direction) {
        case "UPTOWN":
        case 0:
            message.direction = 0;
            break;
        case "DOWNTOWN":
        case 1:
            message.direction = 1;
            break;
        }
        if (object.lineColorHex != null)
            message.lineColorHex = String(object.lineColorHex);
        if (object.arrivals) {
            if (!Array.isArray(object.arrivals))
                throw TypeError(".LineArrivals.arrivals: array expected");
            message.arrivals = [];
            for (var i = 0; i < object.arrivals.length; ++i) {
                if (typeof object.arrivals[i] !== "object")
                    throw TypeError(".LineArrivals.arrivals: object expected");
                message.arrivals[i] = $root.Arrival.fromObject(object.arrivals[i]);
            }
        }
        if (object.debugInfo != null) {
            if (typeof object.debugInfo !== "object")
                throw TypeError(".LineArrivals.debugInfo: object expected");
            message.debugInfo = $root.DebugInfo.fromObject(object.debugInfo);
        }
        return message;
    };

    /**
     * Creates a plain object from a LineArrivals message. Also converts values to other types if specified.
     * @function toObject
     * @memberof LineArrivals
     * @static
     * @param {LineArrivals} message LineArrivals
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    LineArrivals.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.arrivals = [];
        if (options.defaults) {
            object.line = "";
            object.direction = options.enums === String ? "UPTOWN" : 0;
            object.lineColorHex = "";
            object.debugInfo = null;
        }
        if (message.line != null && message.hasOwnProperty("line"))
            object.line = message.line;
        if (message.direction != null && message.hasOwnProperty("direction"))
            object.direction = options.enums === String ? $root.Direction[message.direction] : message.direction;
        if (message.lineColorHex != null && message.hasOwnProperty("lineColorHex"))
            object.lineColorHex = message.lineColorHex;
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            object.debugInfo = $root.DebugInfo.toObject(message.debugInfo, options);
        if (message.arrivals && message.arrivals.length) {
            object.arrivals = [];
            for (var j = 0; j < message.arrivals.length; ++j)
                object.arrivals[j] = $root.Arrival.toObject(message.arrivals[j], options);
        }
        return object;
    };

    /**
     * Converts this LineArrivals to JSON.
     * @function toJSON
     * @memberof LineArrivals
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    LineArrivals.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return LineArrivals;
})();

$root.StationStatus = (function() {

    /**
     * Properties of a StationStatus.
     * @exports IStationStatus
     * @interface IStationStatus
     * @property {string} [name] StationStatus name
     * @property {Array.<ILineArrivals>} [line] StationStatus line
     * @property {number|Long} [dataTimestamp] StationStatus dataTimestamp
     * @property {IDebugInfo} [debugInfo] StationStatus debugInfo
     */

    /**
     * Constructs a new StationStatus.
     * @exports StationStatus
     * @classdesc Represents a StationStatus.
     * @constructor
     * @param {IStationStatus=} [properties] Properties to set
     */
    function StationStatus(properties) {
        this.line = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * StationStatus name.
     * @member {string}name
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.name = "";

    /**
     * StationStatus line.
     * @member {Array.<ILineArrivals>}line
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.line = $util.emptyArray;

    /**
     * StationStatus dataTimestamp.
     * @member {number|Long}dataTimestamp
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.dataTimestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * StationStatus debugInfo.
     * @member {(IDebugInfo|null|undefined)}debugInfo
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.debugInfo = null;

    /**
     * Creates a new StationStatus instance using the specified properties.
     * @function create
     * @memberof StationStatus
     * @static
     * @param {IStationStatus=} [properties] Properties to set
     * @returns {StationStatus} StationStatus instance
     */
    StationStatus.create = function create(properties) {
        return new StationStatus(properties);
    };

    /**
     * Encodes the specified StationStatus message. Does not implicitly {@link StationStatus.verify|verify} messages.
     * @function encode
     * @memberof StationStatus
     * @static
     * @param {IStationStatus} message StationStatus message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    StationStatus.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.name != null && message.hasOwnProperty("name"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
        if (message.line != null && message.line.length)
            for (var i = 0; i < message.line.length; ++i)
                $root.LineArrivals.encode(message.line[i], writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            writer.uint32(/* id 3, wireType 0 =*/24).int64(message.dataTimestamp);
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            $root.DebugInfo.encode(message.debugInfo, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified StationStatus message, length delimited. Does not implicitly {@link StationStatus.verify|verify} messages.
     * @function encodeDelimited
     * @memberof StationStatus
     * @static
     * @param {IStationStatus} message StationStatus message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    StationStatus.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a StationStatus message from the specified reader or buffer.
     * @function decode
     * @memberof StationStatus
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {StationStatus} StationStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    StationStatus.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.StationStatus();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.name = reader.string();
                break;
            case 2:
                if (!(message.line && message.line.length))
                    message.line = [];
                message.line.push($root.LineArrivals.decode(reader, reader.uint32()));
                break;
            case 3:
                message.dataTimestamp = reader.int64();
                break;
            case 4:
                message.debugInfo = $root.DebugInfo.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a StationStatus message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof StationStatus
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {StationStatus} StationStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    StationStatus.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a StationStatus message.
     * @function verify
     * @memberof StationStatus
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    StationStatus.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.name != null && message.hasOwnProperty("name"))
            if (!$util.isString(message.name))
                return "name: string expected";
        if (message.line != null && message.hasOwnProperty("line")) {
            if (!Array.isArray(message.line))
                return "line: array expected";
            for (var i = 0; i < message.line.length; ++i) {
                var error = $root.LineArrivals.verify(message.line[i]);
                if (error)
                    return "line." + error;
            }
        }
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            if (!$util.isInteger(message.dataTimestamp) && !(message.dataTimestamp && $util.isInteger(message.dataTimestamp.low) && $util.isInteger(message.dataTimestamp.high)))
                return "dataTimestamp: integer|Long expected";
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            error = $root.DebugInfo.verify(message.debugInfo);
            if (error)
                return "debugInfo." + error;
        }
        return null;
    };

    /**
     * Creates a StationStatus message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof StationStatus
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {StationStatus} StationStatus
     */
    StationStatus.fromObject = function fromObject(object) {
        if (object instanceof $root.StationStatus)
            return object;
        var message = new $root.StationStatus();
        if (object.name != null)
            message.name = String(object.name);
        if (object.line) {
            if (!Array.isArray(object.line))
                throw TypeError(".StationStatus.line: array expected");
            message.line = [];
            for (var i = 0; i < object.line.length; ++i) {
                if (typeof object.line[i] !== "object")
                    throw TypeError(".StationStatus.line: object expected");
                message.line[i] = $root.LineArrivals.fromObject(object.line[i]);
            }
        }
        if (object.dataTimestamp != null)
            if ($util.Long)
                (message.dataTimestamp = $util.Long.fromValue(object.dataTimestamp)).unsigned = false;
            else if (typeof object.dataTimestamp === "string")
                message.dataTimestamp = parseInt(object.dataTimestamp, 10);
            else if (typeof object.dataTimestamp === "number")
                message.dataTimestamp = object.dataTimestamp;
            else if (typeof object.dataTimestamp === "object")
                message.dataTimestamp = new $util.LongBits(object.dataTimestamp.low >>> 0, object.dataTimestamp.high >>> 0).toNumber();
        if (object.debugInfo != null) {
            if (typeof object.debugInfo !== "object")
                throw TypeError(".StationStatus.debugInfo: object expected");
            message.debugInfo = $root.DebugInfo.fromObject(object.debugInfo);
        }
        return message;
    };

    /**
     * Creates a plain object from a StationStatus message. Also converts values to other types if specified.
     * @function toObject
     * @memberof StationStatus
     * @static
     * @param {StationStatus} message StationStatus
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    StationStatus.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.line = [];
        if (options.defaults) {
            object.name = "";
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.dataTimestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.dataTimestamp = options.longs === String ? "0" : 0;
            object.debugInfo = null;
        }
        if (message.name != null && message.hasOwnProperty("name"))
            object.name = message.name;
        if (message.line && message.line.length) {
            object.line = [];
            for (var j = 0; j < message.line.length; ++j)
                object.line[j] = $root.LineArrivals.toObject(message.line[j], options);
        }
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            if (typeof message.dataTimestamp === "number")
                object.dataTimestamp = options.longs === String ? String(message.dataTimestamp) : message.dataTimestamp;
            else
                object.dataTimestamp = options.longs === String ? $util.Long.prototype.toString.call(message.dataTimestamp) : options.longs === Number ? new $util.LongBits(message.dataTimestamp.low >>> 0, message.dataTimestamp.high >>> 0).toNumber() : message.dataTimestamp;
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            object.debugInfo = $root.DebugInfo.toObject(message.debugInfo, options);
        return object;
    };

    /**
     * Converts this StationStatus to JSON.
     * @function toJSON
     * @memberof StationStatus
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    StationStatus.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return StationStatus;
})();

$root.Station = (function() {

    /**
     * Properties of a Station.
     * @exports IStation
     * @interface IStation
     * @property {string} [id] Station id
     * @property {string} [name] Station name
     */

    /**
     * Constructs a new Station.
     * @exports Station
     * @classdesc Represents a Station.
     * @constructor
     * @param {IStation=} [properties] Properties to set
     */
    function Station(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * Station id.
     * @member {string}id
     * @memberof Station
     * @instance
     */
    Station.prototype.id = "";

    /**
     * Station name.
     * @member {string}name
     * @memberof Station
     * @instance
     */
    Station.prototype.name = "";

    /**
     * Creates a new Station instance using the specified properties.
     * @function create
     * @memberof Station
     * @static
     * @param {IStation=} [properties] Properties to set
     * @returns {Station} Station instance
     */
    Station.create = function create(properties) {
        return new Station(properties);
    };

    /**
     * Encodes the specified Station message. Does not implicitly {@link Station.verify|verify} messages.
     * @function encode
     * @memberof Station
     * @static
     * @param {IStation} message Station message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Station.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.id != null && message.hasOwnProperty("id"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
        if (message.name != null && message.hasOwnProperty("name"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.name);
        return writer;
    };

    /**
     * Encodes the specified Station message, length delimited. Does not implicitly {@link Station.verify|verify} messages.
     * @function encodeDelimited
     * @memberof Station
     * @static
     * @param {IStation} message Station message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Station.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a Station message from the specified reader or buffer.
     * @function decode
     * @memberof Station
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {Station} Station
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Station.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.Station();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.id = reader.string();
                break;
            case 2:
                message.name = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a Station message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof Station
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {Station} Station
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Station.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a Station message.
     * @function verify
     * @memberof Station
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    Station.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.id != null && message.hasOwnProperty("id"))
            if (!$util.isString(message.id))
                return "id: string expected";
        if (message.name != null && message.hasOwnProperty("name"))
            if (!$util.isString(message.name))
                return "name: string expected";
        return null;
    };

    /**
     * Creates a Station message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof Station
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {Station} Station
     */
    Station.fromObject = function fromObject(object) {
        if (object instanceof $root.Station)
            return object;
        var message = new $root.Station();
        if (object.id != null)
            message.id = String(object.id);
        if (object.name != null)
            message.name = String(object.name);
        return message;
    };

    /**
     * Creates a plain object from a Station message. Also converts values to other types if specified.
     * @function toObject
     * @memberof Station
     * @static
     * @param {Station} message Station
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    Station.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.id = "";
            object.name = "";
        }
        if (message.id != null && message.hasOwnProperty("id"))
            object.id = message.id;
        if (message.name != null && message.hasOwnProperty("name"))
            object.name = message.name;
        return object;
    };

    /**
     * Converts this Station to JSON.
     * @function toJSON
     * @memberof Station
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    Station.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return Station;
})();

$root.StationList = (function() {

    /**
     * Properties of a StationList.
     * @exports IStationList
     * @interface IStationList
     * @property {Array.<IStation>} [station] StationList station
     * @property {IDebugInfo} [debugInfo] StationList debugInfo
     */

    /**
     * Constructs a new StationList.
     * @exports StationList
     * @classdesc Represents a StationList.
     * @constructor
     * @param {IStationList=} [properties] Properties to set
     */
    function StationList(properties) {
        this.station = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * StationList station.
     * @member {Array.<IStation>}station
     * @memberof StationList
     * @instance
     */
    StationList.prototype.station = $util.emptyArray;

    /**
     * StationList debugInfo.
     * @member {(IDebugInfo|null|undefined)}debugInfo
     * @memberof StationList
     * @instance
     */
    StationList.prototype.debugInfo = null;

    /**
     * Creates a new StationList instance using the specified properties.
     * @function create
     * @memberof StationList
     * @static
     * @param {IStationList=} [properties] Properties to set
     * @returns {StationList} StationList instance
     */
    StationList.create = function create(properties) {
        return new StationList(properties);
    };

    /**
     * Encodes the specified StationList message. Does not implicitly {@link StationList.verify|verify} messages.
     * @function encode
     * @memberof StationList
     * @static
     * @param {IStationList} message StationList message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    StationList.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.station != null && message.station.length)
            for (var i = 0; i < message.station.length; ++i)
                $root.Station.encode(message.station[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            $root.DebugInfo.encode(message.debugInfo, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified StationList message, length delimited. Does not implicitly {@link StationList.verify|verify} messages.
     * @function encodeDelimited
     * @memberof StationList
     * @static
     * @param {IStationList} message StationList message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    StationList.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a StationList message from the specified reader or buffer.
     * @function decode
     * @memberof StationList
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {StationList} StationList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    StationList.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.StationList();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                if (!(message.station && message.station.length))
                    message.station = [];
                message.station.push($root.Station.decode(reader, reader.uint32()));
                break;
            case 5:
                message.debugInfo = $root.DebugInfo.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a StationList message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof StationList
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {StationList} StationList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    StationList.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a StationList message.
     * @function verify
     * @memberof StationList
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    StationList.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.station != null && message.hasOwnProperty("station")) {
            if (!Array.isArray(message.station))
                return "station: array expected";
            for (var i = 0; i < message.station.length; ++i) {
                var error = $root.Station.verify(message.station[i]);
                if (error)
                    return "station." + error;
            }
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            error = $root.DebugInfo.verify(message.debugInfo);
            if (error)
                return "debugInfo." + error;
        }
        return null;
    };

    /**
     * Creates a StationList message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof StationList
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {StationList} StationList
     */
    StationList.fromObject = function fromObject(object) {
        if (object instanceof $root.StationList)
            return object;
        var message = new $root.StationList();
        if (object.station) {
            if (!Array.isArray(object.station))
                throw TypeError(".StationList.station: array expected");
            message.station = [];
            for (var i = 0; i < object.station.length; ++i) {
                if (typeof object.station[i] !== "object")
                    throw TypeError(".StationList.station: object expected");
                message.station[i] = $root.Station.fromObject(object.station[i]);
            }
        }
        if (object.debugInfo != null) {
            if (typeof object.debugInfo !== "object")
                throw TypeError(".StationList.debugInfo: object expected");
            message.debugInfo = $root.DebugInfo.fromObject(object.debugInfo);
        }
        return message;
    };

    /**
     * Creates a plain object from a StationList message. Also converts values to other types if specified.
     * @function toObject
     * @memberof StationList
     * @static
     * @param {StationList} message StationList
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    StationList.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.station = [];
        if (options.defaults)
            object.debugInfo = null;
        if (message.station && message.station.length) {
            object.station = [];
            for (var j = 0; j < message.station.length; ++j)
                object.station[j] = $root.Station.toObject(message.station[j], options);
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            object.debugInfo = $root.DebugInfo.toObject(message.debugInfo, options);
        return object;
    };

    /**
     * Converts this StationList to JSON.
     * @function toJSON
     * @memberof StationList
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    StationList.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return StationList;
})();

$root.Line = (function() {

    /**
     * Properties of a Line.
     * @exports ILine
     * @interface ILine
     * @property {string} [name] Line name
     * @property {string} [colorHex] Line colorHex
     * @property {boolean} [active] Line active
     */

    /**
     * Constructs a new Line.
     * @exports Line
     * @classdesc Represents a Line.
     * @constructor
     * @param {ILine=} [properties] Properties to set
     */
    function Line(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * Line name.
     * @member {string}name
     * @memberof Line
     * @instance
     */
    Line.prototype.name = "";

    /**
     * Line colorHex.
     * @member {string}colorHex
     * @memberof Line
     * @instance
     */
    Line.prototype.colorHex = "";

    /**
     * Line active.
     * @member {boolean}active
     * @memberof Line
     * @instance
     */
    Line.prototype.active = false;

    /**
     * Creates a new Line instance using the specified properties.
     * @function create
     * @memberof Line
     * @static
     * @param {ILine=} [properties] Properties to set
     * @returns {Line} Line instance
     */
    Line.create = function create(properties) {
        return new Line(properties);
    };

    /**
     * Encodes the specified Line message. Does not implicitly {@link Line.verify|verify} messages.
     * @function encode
     * @memberof Line
     * @static
     * @param {ILine} message Line message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Line.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.name != null && message.hasOwnProperty("name"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.name);
        if (message.colorHex != null && message.hasOwnProperty("colorHex"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.colorHex);
        if (message.active != null && message.hasOwnProperty("active"))
            writer.uint32(/* id 3, wireType 0 =*/24).bool(message.active);
        return writer;
    };

    /**
     * Encodes the specified Line message, length delimited. Does not implicitly {@link Line.verify|verify} messages.
     * @function encodeDelimited
     * @memberof Line
     * @static
     * @param {ILine} message Line message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Line.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a Line message from the specified reader or buffer.
     * @function decode
     * @memberof Line
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {Line} Line
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Line.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.Line();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.name = reader.string();
                break;
            case 2:
                message.colorHex = reader.string();
                break;
            case 3:
                message.active = reader.bool();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a Line message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof Line
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {Line} Line
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Line.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a Line message.
     * @function verify
     * @memberof Line
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    Line.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.name != null && message.hasOwnProperty("name"))
            if (!$util.isString(message.name))
                return "name: string expected";
        if (message.colorHex != null && message.hasOwnProperty("colorHex"))
            if (!$util.isString(message.colorHex))
                return "colorHex: string expected";
        if (message.active != null && message.hasOwnProperty("active"))
            if (typeof message.active !== "boolean")
                return "active: boolean expected";
        return null;
    };

    /**
     * Creates a Line message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof Line
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {Line} Line
     */
    Line.fromObject = function fromObject(object) {
        if (object instanceof $root.Line)
            return object;
        var message = new $root.Line();
        if (object.name != null)
            message.name = String(object.name);
        if (object.colorHex != null)
            message.colorHex = String(object.colorHex);
        if (object.active != null)
            message.active = Boolean(object.active);
        return message;
    };

    /**
     * Creates a plain object from a Line message. Also converts values to other types if specified.
     * @function toObject
     * @memberof Line
     * @static
     * @param {Line} message Line
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    Line.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.name = "";
            object.colorHex = "";
            object.active = false;
        }
        if (message.name != null && message.hasOwnProperty("name"))
            object.name = message.name;
        if (message.colorHex != null && message.hasOwnProperty("colorHex"))
            object.colorHex = message.colorHex;
        if (message.active != null && message.hasOwnProperty("active"))
            object.active = message.active;
        return object;
    };

    /**
     * Converts this Line to JSON.
     * @function toJSON
     * @memberof Line
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    Line.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return Line;
})();

$root.LineList = (function() {

    /**
     * Properties of a LineList.
     * @exports ILineList
     * @interface ILineList
     * @property {Array.<ILine>} [line] LineList line
     * @property {IDebugInfo} [debugInfo] LineList debugInfo
     */

    /**
     * Constructs a new LineList.
     * @exports LineList
     * @classdesc Represents a LineList.
     * @constructor
     * @param {ILineList=} [properties] Properties to set
     */
    function LineList(properties) {
        this.line = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * LineList line.
     * @member {Array.<ILine>}line
     * @memberof LineList
     * @instance
     */
    LineList.prototype.line = $util.emptyArray;

    /**
     * LineList debugInfo.
     * @member {(IDebugInfo|null|undefined)}debugInfo
     * @memberof LineList
     * @instance
     */
    LineList.prototype.debugInfo = null;

    /**
     * Creates a new LineList instance using the specified properties.
     * @function create
     * @memberof LineList
     * @static
     * @param {ILineList=} [properties] Properties to set
     * @returns {LineList} LineList instance
     */
    LineList.create = function create(properties) {
        return new LineList(properties);
    };

    /**
     * Encodes the specified LineList message. Does not implicitly {@link LineList.verify|verify} messages.
     * @function encode
     * @memberof LineList
     * @static
     * @param {ILineList} message LineList message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineList.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.line != null && message.line.length)
            for (var i = 0; i < message.line.length; ++i)
                $root.Line.encode(message.line[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            $root.DebugInfo.encode(message.debugInfo, writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified LineList message, length delimited. Does not implicitly {@link LineList.verify|verify} messages.
     * @function encodeDelimited
     * @memberof LineList
     * @static
     * @param {ILineList} message LineList message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineList.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a LineList message from the specified reader or buffer.
     * @function decode
     * @memberof LineList
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {LineList} LineList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineList.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.LineList();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                if (!(message.line && message.line.length))
                    message.line = [];
                message.line.push($root.Line.decode(reader, reader.uint32()));
                break;
            case 4:
                message.debugInfo = $root.DebugInfo.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a LineList message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof LineList
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {LineList} LineList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineList.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a LineList message.
     * @function verify
     * @memberof LineList
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    LineList.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.line != null && message.hasOwnProperty("line")) {
            if (!Array.isArray(message.line))
                return "line: array expected";
            for (var i = 0; i < message.line.length; ++i) {
                var error = $root.Line.verify(message.line[i]);
                if (error)
                    return "line." + error;
            }
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            error = $root.DebugInfo.verify(message.debugInfo);
            if (error)
                return "debugInfo." + error;
        }
        return null;
    };

    /**
     * Creates a LineList message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof LineList
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {LineList} LineList
     */
    LineList.fromObject = function fromObject(object) {
        if (object instanceof $root.LineList)
            return object;
        var message = new $root.LineList();
        if (object.line) {
            if (!Array.isArray(object.line))
                throw TypeError(".LineList.line: array expected");
            message.line = [];
            for (var i = 0; i < object.line.length; ++i) {
                if (typeof object.line[i] !== "object")
                    throw TypeError(".LineList.line: object expected");
                message.line[i] = $root.Line.fromObject(object.line[i]);
            }
        }
        if (object.debugInfo != null) {
            if (typeof object.debugInfo !== "object")
                throw TypeError(".LineList.debugInfo: object expected");
            message.debugInfo = $root.DebugInfo.fromObject(object.debugInfo);
        }
        return message;
    };

    /**
     * Creates a plain object from a LineList message. Also converts values to other types if specified.
     * @function toObject
     * @memberof LineList
     * @static
     * @param {LineList} message LineList
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    LineList.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.line = [];
        if (options.defaults)
            object.debugInfo = null;
        if (message.line && message.line.length) {
            object.line = [];
            for (var j = 0; j < message.line.length; ++j)
                object.line[j] = $root.Line.toObject(message.line[j], options);
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            object.debugInfo = $root.DebugInfo.toObject(message.debugInfo, options);
        return object;
    };

    /**
     * Converts this LineList to JSON.
     * @function toJSON
     * @memberof LineList
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    LineList.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return LineList;
})();

module.exports = $root;
