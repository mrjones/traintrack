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

/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
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
     * @property {number|Long|null} [processingTimeMs] DebugInfo processingTimeMs
     * @property {string|null} [buildVersion] DebugInfo buildVersion
     * @property {number|Long|null} [buildTimestamp] DebugInfo buildTimestamp
     */

    /**
     * Constructs a new DebugInfo.
     * @exports DebugInfo
     * @classdesc Represents a DebugInfo.
     * @implements IDebugInfo
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
     * @member {number|Long} processingTimeMs
     * @memberof DebugInfo
     * @instance
     */
    DebugInfo.prototype.processingTimeMs = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * DebugInfo buildVersion.
     * @member {string} buildVersion
     * @memberof DebugInfo
     * @instance
     */
    DebugInfo.prototype.buildVersion = "";

    /**
     * DebugInfo buildTimestamp.
     * @member {number|Long} buildTimestamp
     * @memberof DebugInfo
     * @instance
     */
    DebugInfo.prototype.buildTimestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

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
        if (message.buildVersion != null && message.hasOwnProperty("buildVersion"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.buildVersion);
        if (message.buildTimestamp != null && message.hasOwnProperty("buildTimestamp"))
            writer.uint32(/* id 3, wireType 0 =*/24).int64(message.buildTimestamp);
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
            case 2:
                message.buildVersion = reader.string();
                break;
            case 3:
                message.buildTimestamp = reader.int64();
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
        if (message.buildVersion != null && message.hasOwnProperty("buildVersion"))
            if (!$util.isString(message.buildVersion))
                return "buildVersion: string expected";
        if (message.buildTimestamp != null && message.hasOwnProperty("buildTimestamp"))
            if (!$util.isInteger(message.buildTimestamp) && !(message.buildTimestamp && $util.isInteger(message.buildTimestamp.low) && $util.isInteger(message.buildTimestamp.high)))
                return "buildTimestamp: integer|Long expected";
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
        if (object.buildVersion != null)
            message.buildVersion = String(object.buildVersion);
        if (object.buildTimestamp != null)
            if ($util.Long)
                (message.buildTimestamp = $util.Long.fromValue(object.buildTimestamp)).unsigned = false;
            else if (typeof object.buildTimestamp === "string")
                message.buildTimestamp = parseInt(object.buildTimestamp, 10);
            else if (typeof object.buildTimestamp === "number")
                message.buildTimestamp = object.buildTimestamp;
            else if (typeof object.buildTimestamp === "object")
                message.buildTimestamp = new $util.LongBits(object.buildTimestamp.low >>> 0, object.buildTimestamp.high >>> 0).toNumber();
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
        if (options.defaults) {
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.processingTimeMs = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.processingTimeMs = options.longs === String ? "0" : 0;
            object.buildVersion = "";
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.buildTimestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.buildTimestamp = options.longs === String ? "0" : 0;
        }
        if (message.processingTimeMs != null && message.hasOwnProperty("processingTimeMs"))
            if (typeof message.processingTimeMs === "number")
                object.processingTimeMs = options.longs === String ? String(message.processingTimeMs) : message.processingTimeMs;
            else
                object.processingTimeMs = options.longs === String ? $util.Long.prototype.toString.call(message.processingTimeMs) : options.longs === Number ? new $util.LongBits(message.processingTimeMs.low >>> 0, message.processingTimeMs.high >>> 0).toNumber() : message.processingTimeMs;
        if (message.buildVersion != null && message.hasOwnProperty("buildVersion"))
            object.buildVersion = message.buildVersion;
        if (message.buildTimestamp != null && message.hasOwnProperty("buildTimestamp"))
            if (typeof message.buildTimestamp === "number")
                object.buildTimestamp = options.longs === String ? String(message.buildTimestamp) : message.buildTimestamp;
            else
                object.buildTimestamp = options.longs === String ? $util.Long.prototype.toString.call(message.buildTimestamp) : options.longs === Number ? new $util.LongBits(message.buildTimestamp.low >>> 0, message.buildTimestamp.high >>> 0).toNumber() : message.buildTimestamp;
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

$root.LineArrival = (function() {

    /**
     * Properties of a LineArrival.
     * @exports ILineArrival
     * @interface ILineArrival
     * @property {number|Long|null} [timestamp] LineArrival timestamp
     * @property {string|null} [tripId] LineArrival tripId
     * @property {string|null} [headsign] LineArrival headsign
     */

    /**
     * Constructs a new LineArrival.
     * @exports LineArrival
     * @classdesc Represents a LineArrival.
     * @implements ILineArrival
     * @constructor
     * @param {ILineArrival=} [properties] Properties to set
     */
    function LineArrival(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * LineArrival timestamp.
     * @member {number|Long} timestamp
     * @memberof LineArrival
     * @instance
     */
    LineArrival.prototype.timestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * LineArrival tripId.
     * @member {string} tripId
     * @memberof LineArrival
     * @instance
     */
    LineArrival.prototype.tripId = "";

    /**
     * LineArrival headsign.
     * @member {string} headsign
     * @memberof LineArrival
     * @instance
     */
    LineArrival.prototype.headsign = "";

    /**
     * Creates a new LineArrival instance using the specified properties.
     * @function create
     * @memberof LineArrival
     * @static
     * @param {ILineArrival=} [properties] Properties to set
     * @returns {LineArrival} LineArrival instance
     */
    LineArrival.create = function create(properties) {
        return new LineArrival(properties);
    };

    /**
     * Encodes the specified LineArrival message. Does not implicitly {@link LineArrival.verify|verify} messages.
     * @function encode
     * @memberof LineArrival
     * @static
     * @param {ILineArrival} message LineArrival message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineArrival.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            writer.uint32(/* id 1, wireType 0 =*/8).int64(message.timestamp);
        if (message.tripId != null && message.hasOwnProperty("tripId"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.tripId);
        if (message.headsign != null && message.hasOwnProperty("headsign"))
            writer.uint32(/* id 3, wireType 2 =*/26).string(message.headsign);
        return writer;
    };

    /**
     * Encodes the specified LineArrival message, length delimited. Does not implicitly {@link LineArrival.verify|verify} messages.
     * @function encodeDelimited
     * @memberof LineArrival
     * @static
     * @param {ILineArrival} message LineArrival message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineArrival.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a LineArrival message from the specified reader or buffer.
     * @function decode
     * @memberof LineArrival
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {LineArrival} LineArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineArrival.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.LineArrival();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.timestamp = reader.int64();
                break;
            case 2:
                message.tripId = reader.string();
                break;
            case 3:
                message.headsign = reader.string();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a LineArrival message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof LineArrival
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {LineArrival} LineArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineArrival.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a LineArrival message.
     * @function verify
     * @memberof LineArrival
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    LineArrival.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            if (!$util.isInteger(message.timestamp) && !(message.timestamp && $util.isInteger(message.timestamp.low) && $util.isInteger(message.timestamp.high)))
                return "timestamp: integer|Long expected";
        if (message.tripId != null && message.hasOwnProperty("tripId"))
            if (!$util.isString(message.tripId))
                return "tripId: string expected";
        if (message.headsign != null && message.hasOwnProperty("headsign"))
            if (!$util.isString(message.headsign))
                return "headsign: string expected";
        return null;
    };

    /**
     * Creates a LineArrival message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof LineArrival
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {LineArrival} LineArrival
     */
    LineArrival.fromObject = function fromObject(object) {
        if (object instanceof $root.LineArrival)
            return object;
        var message = new $root.LineArrival();
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
        if (object.headsign != null)
            message.headsign = String(object.headsign);
        return message;
    };

    /**
     * Creates a plain object from a LineArrival message. Also converts values to other types if specified.
     * @function toObject
     * @memberof LineArrival
     * @static
     * @param {LineArrival} message LineArrival
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    LineArrival.toObject = function toObject(message, options) {
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
            object.headsign = "";
        }
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            if (typeof message.timestamp === "number")
                object.timestamp = options.longs === String ? String(message.timestamp) : message.timestamp;
            else
                object.timestamp = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp) : options.longs === Number ? new $util.LongBits(message.timestamp.low >>> 0, message.timestamp.high >>> 0).toNumber() : message.timestamp;
        if (message.tripId != null && message.hasOwnProperty("tripId"))
            object.tripId = message.tripId;
        if (message.headsign != null && message.hasOwnProperty("headsign"))
            object.headsign = message.headsign;
        return object;
    };

    /**
     * Converts this LineArrival to JSON.
     * @function toJSON
     * @memberof LineArrival
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    LineArrival.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return LineArrival;
})();

$root.LineArrivals = (function() {

    /**
     * Properties of a LineArrivals.
     * @exports ILineArrivals
     * @interface ILineArrivals
     * @property {string|null} [line] LineArrivals line
     * @property {Direction|null} [direction] LineArrivals direction
     * @property {string|null} [lineColorHex] LineArrivals lineColorHex
     * @property {Array.<ILineArrival>|null} [arrivals] LineArrivals arrivals
     * @property {IDebugInfo|null} [debugInfo] LineArrivals debugInfo
     */

    /**
     * Constructs a new LineArrivals.
     * @exports LineArrivals
     * @classdesc Represents a LineArrivals.
     * @implements ILineArrivals
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
     * @member {string} line
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.line = "";

    /**
     * LineArrivals direction.
     * @member {Direction} direction
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.direction = 0;

    /**
     * LineArrivals lineColorHex.
     * @member {string} lineColorHex
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.lineColorHex = "";

    /**
     * LineArrivals arrivals.
     * @member {Array.<ILineArrival>} arrivals
     * @memberof LineArrivals
     * @instance
     */
    LineArrivals.prototype.arrivals = $util.emptyArray;

    /**
     * LineArrivals debugInfo.
     * @member {IDebugInfo|null|undefined} debugInfo
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
                $root.LineArrival.encode(message.arrivals[i], writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
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
                message.arrivals.push($root.LineArrival.decode(reader, reader.uint32()));
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
                var error = $root.LineArrival.verify(message.arrivals[i]);
                if (error)
                    return "arrivals." + error;
            }
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            var error = $root.DebugInfo.verify(message.debugInfo);
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
                message.arrivals[i] = $root.LineArrival.fromObject(object.arrivals[i]);
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
                object.arrivals[j] = $root.LineArrival.toObject(message.arrivals[j], options);
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

$root.TrainItineraryArrival = (function() {

    /**
     * Properties of a TrainItineraryArrival.
     * @exports ITrainItineraryArrival
     * @interface ITrainItineraryArrival
     * @property {number|Long|null} [timestamp] TrainItineraryArrival timestamp
     * @property {IStation|null} [station] TrainItineraryArrival station
     */

    /**
     * Constructs a new TrainItineraryArrival.
     * @exports TrainItineraryArrival
     * @classdesc Represents a TrainItineraryArrival.
     * @implements ITrainItineraryArrival
     * @constructor
     * @param {ITrainItineraryArrival=} [properties] Properties to set
     */
    function TrainItineraryArrival(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * TrainItineraryArrival timestamp.
     * @member {number|Long} timestamp
     * @memberof TrainItineraryArrival
     * @instance
     */
    TrainItineraryArrival.prototype.timestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * TrainItineraryArrival station.
     * @member {IStation|null|undefined} station
     * @memberof TrainItineraryArrival
     * @instance
     */
    TrainItineraryArrival.prototype.station = null;

    /**
     * Creates a new TrainItineraryArrival instance using the specified properties.
     * @function create
     * @memberof TrainItineraryArrival
     * @static
     * @param {ITrainItineraryArrival=} [properties] Properties to set
     * @returns {TrainItineraryArrival} TrainItineraryArrival instance
     */
    TrainItineraryArrival.create = function create(properties) {
        return new TrainItineraryArrival(properties);
    };

    /**
     * Encodes the specified TrainItineraryArrival message. Does not implicitly {@link TrainItineraryArrival.verify|verify} messages.
     * @function encode
     * @memberof TrainItineraryArrival
     * @static
     * @param {ITrainItineraryArrival} message TrainItineraryArrival message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainItineraryArrival.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            writer.uint32(/* id 1, wireType 0 =*/8).int64(message.timestamp);
        if (message.station != null && message.hasOwnProperty("station"))
            $root.Station.encode(message.station, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified TrainItineraryArrival message, length delimited. Does not implicitly {@link TrainItineraryArrival.verify|verify} messages.
     * @function encodeDelimited
     * @memberof TrainItineraryArrival
     * @static
     * @param {ITrainItineraryArrival} message TrainItineraryArrival message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainItineraryArrival.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a TrainItineraryArrival message from the specified reader or buffer.
     * @function decode
     * @memberof TrainItineraryArrival
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {TrainItineraryArrival} TrainItineraryArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainItineraryArrival.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.TrainItineraryArrival();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.timestamp = reader.int64();
                break;
            case 2:
                message.station = $root.Station.decode(reader, reader.uint32());
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a TrainItineraryArrival message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof TrainItineraryArrival
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {TrainItineraryArrival} TrainItineraryArrival
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainItineraryArrival.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a TrainItineraryArrival message.
     * @function verify
     * @memberof TrainItineraryArrival
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    TrainItineraryArrival.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            if (!$util.isInteger(message.timestamp) && !(message.timestamp && $util.isInteger(message.timestamp.low) && $util.isInteger(message.timestamp.high)))
                return "timestamp: integer|Long expected";
        if (message.station != null && message.hasOwnProperty("station")) {
            var error = $root.Station.verify(message.station);
            if (error)
                return "station." + error;
        }
        return null;
    };

    /**
     * Creates a TrainItineraryArrival message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof TrainItineraryArrival
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {TrainItineraryArrival} TrainItineraryArrival
     */
    TrainItineraryArrival.fromObject = function fromObject(object) {
        if (object instanceof $root.TrainItineraryArrival)
            return object;
        var message = new $root.TrainItineraryArrival();
        if (object.timestamp != null)
            if ($util.Long)
                (message.timestamp = $util.Long.fromValue(object.timestamp)).unsigned = false;
            else if (typeof object.timestamp === "string")
                message.timestamp = parseInt(object.timestamp, 10);
            else if (typeof object.timestamp === "number")
                message.timestamp = object.timestamp;
            else if (typeof object.timestamp === "object")
                message.timestamp = new $util.LongBits(object.timestamp.low >>> 0, object.timestamp.high >>> 0).toNumber();
        if (object.station != null) {
            if (typeof object.station !== "object")
                throw TypeError(".TrainItineraryArrival.station: object expected");
            message.station = $root.Station.fromObject(object.station);
        }
        return message;
    };

    /**
     * Creates a plain object from a TrainItineraryArrival message. Also converts values to other types if specified.
     * @function toObject
     * @memberof TrainItineraryArrival
     * @static
     * @param {TrainItineraryArrival} message TrainItineraryArrival
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    TrainItineraryArrival.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.timestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.timestamp = options.longs === String ? "0" : 0;
            object.station = null;
        }
        if (message.timestamp != null && message.hasOwnProperty("timestamp"))
            if (typeof message.timestamp === "number")
                object.timestamp = options.longs === String ? String(message.timestamp) : message.timestamp;
            else
                object.timestamp = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp) : options.longs === Number ? new $util.LongBits(message.timestamp.low >>> 0, message.timestamp.high >>> 0).toNumber() : message.timestamp;
        if (message.station != null && message.hasOwnProperty("station"))
            object.station = $root.Station.toObject(message.station, options);
        return object;
    };

    /**
     * Converts this TrainItineraryArrival to JSON.
     * @function toJSON
     * @memberof TrainItineraryArrival
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    TrainItineraryArrival.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return TrainItineraryArrival;
})();

$root.TrainItinerary = (function() {

    /**
     * Properties of a TrainItinerary.
     * @exports ITrainItinerary
     * @interface ITrainItinerary
     * @property {string|null} [line] TrainItinerary line
     * @property {Direction|null} [direction] TrainItinerary direction
     * @property {string|null} [lineColorHex] TrainItinerary lineColorHex
     * @property {Array.<ITrainItineraryArrival>|null} [arrival] TrainItinerary arrival
     * @property {number|Long|null} [dataTimestamp] TrainItinerary dataTimestamp
     * @property {IDebugInfo|null} [debugInfo] TrainItinerary debugInfo
     */

    /**
     * Constructs a new TrainItinerary.
     * @exports TrainItinerary
     * @classdesc Represents a TrainItinerary.
     * @implements ITrainItinerary
     * @constructor
     * @param {ITrainItinerary=} [properties] Properties to set
     */
    function TrainItinerary(properties) {
        this.arrival = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * TrainItinerary line.
     * @member {string} line
     * @memberof TrainItinerary
     * @instance
     */
    TrainItinerary.prototype.line = "";

    /**
     * TrainItinerary direction.
     * @member {Direction} direction
     * @memberof TrainItinerary
     * @instance
     */
    TrainItinerary.prototype.direction = 0;

    /**
     * TrainItinerary lineColorHex.
     * @member {string} lineColorHex
     * @memberof TrainItinerary
     * @instance
     */
    TrainItinerary.prototype.lineColorHex = "";

    /**
     * TrainItinerary arrival.
     * @member {Array.<ITrainItineraryArrival>} arrival
     * @memberof TrainItinerary
     * @instance
     */
    TrainItinerary.prototype.arrival = $util.emptyArray;

    /**
     * TrainItinerary dataTimestamp.
     * @member {number|Long} dataTimestamp
     * @memberof TrainItinerary
     * @instance
     */
    TrainItinerary.prototype.dataTimestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * TrainItinerary debugInfo.
     * @member {IDebugInfo|null|undefined} debugInfo
     * @memberof TrainItinerary
     * @instance
     */
    TrainItinerary.prototype.debugInfo = null;

    /**
     * Creates a new TrainItinerary instance using the specified properties.
     * @function create
     * @memberof TrainItinerary
     * @static
     * @param {ITrainItinerary=} [properties] Properties to set
     * @returns {TrainItinerary} TrainItinerary instance
     */
    TrainItinerary.create = function create(properties) {
        return new TrainItinerary(properties);
    };

    /**
     * Encodes the specified TrainItinerary message. Does not implicitly {@link TrainItinerary.verify|verify} messages.
     * @function encode
     * @memberof TrainItinerary
     * @static
     * @param {ITrainItinerary} message TrainItinerary message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainItinerary.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.line != null && message.hasOwnProperty("line"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.line);
        if (message.direction != null && message.hasOwnProperty("direction"))
            writer.uint32(/* id 2, wireType 0 =*/16).int32(message.direction);
        if (message.lineColorHex != null && message.hasOwnProperty("lineColorHex"))
            writer.uint32(/* id 3, wireType 2 =*/26).string(message.lineColorHex);
        if (message.arrival != null && message.arrival.length)
            for (var i = 0; i < message.arrival.length; ++i)
                $root.TrainItineraryArrival.encode(message.arrival[i], writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            $root.DebugInfo.encode(message.debugInfo, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            writer.uint32(/* id 6, wireType 0 =*/48).int64(message.dataTimestamp);
        return writer;
    };

    /**
     * Encodes the specified TrainItinerary message, length delimited. Does not implicitly {@link TrainItinerary.verify|verify} messages.
     * @function encodeDelimited
     * @memberof TrainItinerary
     * @static
     * @param {ITrainItinerary} message TrainItinerary message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainItinerary.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a TrainItinerary message from the specified reader or buffer.
     * @function decode
     * @memberof TrainItinerary
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {TrainItinerary} TrainItinerary
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainItinerary.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.TrainItinerary();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.line = reader.string();
                break;
            case 2:
                message.direction = reader.int32();
                break;
            case 3:
                message.lineColorHex = reader.string();
                break;
            case 4:
                if (!(message.arrival && message.arrival.length))
                    message.arrival = [];
                message.arrival.push($root.TrainItineraryArrival.decode(reader, reader.uint32()));
                break;
            case 6:
                message.dataTimestamp = reader.int64();
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
     * Decodes a TrainItinerary message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof TrainItinerary
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {TrainItinerary} TrainItinerary
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainItinerary.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a TrainItinerary message.
     * @function verify
     * @memberof TrainItinerary
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    TrainItinerary.verify = function verify(message) {
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
        if (message.arrival != null && message.hasOwnProperty("arrival")) {
            if (!Array.isArray(message.arrival))
                return "arrival: array expected";
            for (var i = 0; i < message.arrival.length; ++i) {
                var error = $root.TrainItineraryArrival.verify(message.arrival[i]);
                if (error)
                    return "arrival." + error;
            }
        }
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            if (!$util.isInteger(message.dataTimestamp) && !(message.dataTimestamp && $util.isInteger(message.dataTimestamp.low) && $util.isInteger(message.dataTimestamp.high)))
                return "dataTimestamp: integer|Long expected";
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            var error = $root.DebugInfo.verify(message.debugInfo);
            if (error)
                return "debugInfo." + error;
        }
        return null;
    };

    /**
     * Creates a TrainItinerary message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof TrainItinerary
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {TrainItinerary} TrainItinerary
     */
    TrainItinerary.fromObject = function fromObject(object) {
        if (object instanceof $root.TrainItinerary)
            return object;
        var message = new $root.TrainItinerary();
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
        if (object.arrival) {
            if (!Array.isArray(object.arrival))
                throw TypeError(".TrainItinerary.arrival: array expected");
            message.arrival = [];
            for (var i = 0; i < object.arrival.length; ++i) {
                if (typeof object.arrival[i] !== "object")
                    throw TypeError(".TrainItinerary.arrival: object expected");
                message.arrival[i] = $root.TrainItineraryArrival.fromObject(object.arrival[i]);
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
                throw TypeError(".TrainItinerary.debugInfo: object expected");
            message.debugInfo = $root.DebugInfo.fromObject(object.debugInfo);
        }
        return message;
    };

    /**
     * Creates a plain object from a TrainItinerary message. Also converts values to other types if specified.
     * @function toObject
     * @memberof TrainItinerary
     * @static
     * @param {TrainItinerary} message TrainItinerary
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    TrainItinerary.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.arrival = [];
        if (options.defaults) {
            object.line = "";
            object.direction = options.enums === String ? "UPTOWN" : 0;
            object.lineColorHex = "";
            object.debugInfo = null;
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.dataTimestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.dataTimestamp = options.longs === String ? "0" : 0;
        }
        if (message.line != null && message.hasOwnProperty("line"))
            object.line = message.line;
        if (message.direction != null && message.hasOwnProperty("direction"))
            object.direction = options.enums === String ? $root.Direction[message.direction] : message.direction;
        if (message.lineColorHex != null && message.hasOwnProperty("lineColorHex"))
            object.lineColorHex = message.lineColorHex;
        if (message.arrival && message.arrival.length) {
            object.arrival = [];
            for (var j = 0; j < message.arrival.length; ++j)
                object.arrival[j] = $root.TrainItineraryArrival.toObject(message.arrival[j], options);
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            object.debugInfo = $root.DebugInfo.toObject(message.debugInfo, options);
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            if (typeof message.dataTimestamp === "number")
                object.dataTimestamp = options.longs === String ? String(message.dataTimestamp) : message.dataTimestamp;
            else
                object.dataTimestamp = options.longs === String ? $util.Long.prototype.toString.call(message.dataTimestamp) : options.longs === Number ? new $util.LongBits(message.dataTimestamp.low >>> 0, message.dataTimestamp.high >>> 0).toNumber() : message.dataTimestamp;
        return object;
    };

    /**
     * Converts this TrainItinerary to JSON.
     * @function toJSON
     * @memberof TrainItinerary
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    TrainItinerary.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return TrainItinerary;
})();

$root.StationStatus = (function() {

    /**
     * Properties of a StationStatus.
     * @exports IStationStatus
     * @interface IStationStatus
     * @property {string|null} [name] StationStatus name
     * @property {string|null} [id] StationStatus id
     * @property {Array.<ILineArrivals>|null} [line] StationStatus line
     * @property {number|Long|null} [dataTimestamp] StationStatus dataTimestamp
     * @property {Array.<ISubwayStatusMessage>|null} [statusMessage] StationStatus statusMessage
     * @property {IDebugInfo|null} [debugInfo] StationStatus debugInfo
     */

    /**
     * Constructs a new StationStatus.
     * @exports StationStatus
     * @classdesc Represents a StationStatus.
     * @implements IStationStatus
     * @constructor
     * @param {IStationStatus=} [properties] Properties to set
     */
    function StationStatus(properties) {
        this.line = [];
        this.statusMessage = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * StationStatus name.
     * @member {string} name
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.name = "";

    /**
     * StationStatus id.
     * @member {string} id
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.id = "";

    /**
     * StationStatus line.
     * @member {Array.<ILineArrivals>} line
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.line = $util.emptyArray;

    /**
     * StationStatus dataTimestamp.
     * @member {number|Long} dataTimestamp
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.dataTimestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * StationStatus statusMessage.
     * @member {Array.<ISubwayStatusMessage>} statusMessage
     * @memberof StationStatus
     * @instance
     */
    StationStatus.prototype.statusMessage = $util.emptyArray;

    /**
     * StationStatus debugInfo.
     * @member {IDebugInfo|null|undefined} debugInfo
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
        if (message.id != null && message.hasOwnProperty("id"))
            writer.uint32(/* id 5, wireType 2 =*/42).string(message.id);
        if (message.statusMessage != null && message.statusMessage.length)
            for (var i = 0; i < message.statusMessage.length; ++i)
                $root.SubwayStatusMessage.encode(message.statusMessage[i], writer.uint32(/* id 6, wireType 2 =*/50).fork()).ldelim();
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
            case 5:
                message.id = reader.string();
                break;
            case 2:
                if (!(message.line && message.line.length))
                    message.line = [];
                message.line.push($root.LineArrivals.decode(reader, reader.uint32()));
                break;
            case 3:
                message.dataTimestamp = reader.int64();
                break;
            case 6:
                if (!(message.statusMessage && message.statusMessage.length))
                    message.statusMessage = [];
                message.statusMessage.push($root.SubwayStatusMessage.decode(reader, reader.uint32()));
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
        if (message.id != null && message.hasOwnProperty("id"))
            if (!$util.isString(message.id))
                return "id: string expected";
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
        if (message.statusMessage != null && message.hasOwnProperty("statusMessage")) {
            if (!Array.isArray(message.statusMessage))
                return "statusMessage: array expected";
            for (var i = 0; i < message.statusMessage.length; ++i) {
                var error = $root.SubwayStatusMessage.verify(message.statusMessage[i]);
                if (error)
                    return "statusMessage." + error;
            }
        }
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            var error = $root.DebugInfo.verify(message.debugInfo);
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
        if (object.id != null)
            message.id = String(object.id);
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
        if (object.statusMessage) {
            if (!Array.isArray(object.statusMessage))
                throw TypeError(".StationStatus.statusMessage: array expected");
            message.statusMessage = [];
            for (var i = 0; i < object.statusMessage.length; ++i) {
                if (typeof object.statusMessage[i] !== "object")
                    throw TypeError(".StationStatus.statusMessage: object expected");
                message.statusMessage[i] = $root.SubwayStatusMessage.fromObject(object.statusMessage[i]);
            }
        }
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
        if (options.arrays || options.defaults) {
            object.line = [];
            object.statusMessage = [];
        }
        if (options.defaults) {
            object.name = "";
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.dataTimestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.dataTimestamp = options.longs === String ? "0" : 0;
            object.debugInfo = null;
            object.id = "";
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
        if (message.id != null && message.hasOwnProperty("id"))
            object.id = message.id;
        if (message.statusMessage && message.statusMessage.length) {
            object.statusMessage = [];
            for (var j = 0; j < message.statusMessage.length; ++j)
                object.statusMessage[j] = $root.SubwayStatusMessage.toObject(message.statusMessage[j], options);
        }
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
     * @property {string|null} [id] Station id
     * @property {string|null} [name] Station name
     * @property {Array.<string>|null} [lines] Station lines
     */

    /**
     * Constructs a new Station.
     * @exports Station
     * @classdesc Represents a Station.
     * @implements IStation
     * @constructor
     * @param {IStation=} [properties] Properties to set
     */
    function Station(properties) {
        this.lines = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * Station id.
     * @member {string} id
     * @memberof Station
     * @instance
     */
    Station.prototype.id = "";

    /**
     * Station name.
     * @member {string} name
     * @memberof Station
     * @instance
     */
    Station.prototype.name = "";

    /**
     * Station lines.
     * @member {Array.<string>} lines
     * @memberof Station
     * @instance
     */
    Station.prototype.lines = $util.emptyArray;

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
        if (message.lines != null && message.lines.length)
            for (var i = 0; i < message.lines.length; ++i)
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.lines[i]);
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
            case 3:
                if (!(message.lines && message.lines.length))
                    message.lines = [];
                message.lines.push(reader.string());
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
        if (message.lines != null && message.hasOwnProperty("lines")) {
            if (!Array.isArray(message.lines))
                return "lines: array expected";
            for (var i = 0; i < message.lines.length; ++i)
                if (!$util.isString(message.lines[i]))
                    return "lines: string[] expected";
        }
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
        if (object.lines) {
            if (!Array.isArray(object.lines))
                throw TypeError(".Station.lines: array expected");
            message.lines = [];
            for (var i = 0; i < object.lines.length; ++i)
                message.lines[i] = String(object.lines[i]);
        }
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
        if (options.arrays || options.defaults)
            object.lines = [];
        if (options.defaults) {
            object.id = "";
            object.name = "";
        }
        if (message.id != null && message.hasOwnProperty("id"))
            object.id = message.id;
        if (message.name != null && message.hasOwnProperty("name"))
            object.name = message.name;
        if (message.lines && message.lines.length) {
            object.lines = [];
            for (var j = 0; j < message.lines.length; ++j)
                object.lines[j] = message.lines[j];
        }
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
     * @property {Array.<IStation>|null} [station] StationList station
     * @property {IDebugInfo|null} [debugInfo] StationList debugInfo
     */

    /**
     * Constructs a new StationList.
     * @exports StationList
     * @classdesc Represents a StationList.
     * @implements IStationList
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
     * @member {Array.<IStation>} station
     * @memberof StationList
     * @instance
     */
    StationList.prototype.station = $util.emptyArray;

    /**
     * StationList debugInfo.
     * @member {IDebugInfo|null|undefined} debugInfo
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
            var error = $root.DebugInfo.verify(message.debugInfo);
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
     * @property {string|null} [name] Line name
     * @property {string|null} [colorHex] Line colorHex
     * @property {boolean|null} [active] Line active
     */

    /**
     * Constructs a new Line.
     * @exports Line
     * @classdesc Represents a Line.
     * @implements ILine
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
     * @member {string} name
     * @memberof Line
     * @instance
     */
    Line.prototype.name = "";

    /**
     * Line colorHex.
     * @member {string} colorHex
     * @memberof Line
     * @instance
     */
    Line.prototype.colorHex = "";

    /**
     * Line active.
     * @member {boolean} active
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
     * @property {Array.<ILine>|null} [line] LineList line
     * @property {IDebugInfo|null} [debugInfo] LineList debugInfo
     */

    /**
     * Constructs a new LineList.
     * @exports LineList
     * @classdesc Represents a LineList.
     * @implements ILineList
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
     * @member {Array.<ILine>} line
     * @memberof LineList
     * @instance
     */
    LineList.prototype.line = $util.emptyArray;

    /**
     * LineList debugInfo.
     * @member {IDebugInfo|null|undefined} debugInfo
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
            var error = $root.DebugInfo.verify(message.debugInfo);
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

$root.TrainArrivalHistoryEntry = (function() {

    /**
     * Properties of a TrainArrivalHistoryEntry.
     * @exports ITrainArrivalHistoryEntry
     * @interface ITrainArrivalHistoryEntry
     * @property {number|Long|null} [dataTimestamp] TrainArrivalHistoryEntry dataTimestamp
     * @property {number|Long|null} [arrivalTime] TrainArrivalHistoryEntry arrivalTime
     */

    /**
     * Constructs a new TrainArrivalHistoryEntry.
     * @exports TrainArrivalHistoryEntry
     * @classdesc Represents a TrainArrivalHistoryEntry.
     * @implements ITrainArrivalHistoryEntry
     * @constructor
     * @param {ITrainArrivalHistoryEntry=} [properties] Properties to set
     */
    function TrainArrivalHistoryEntry(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * TrainArrivalHistoryEntry dataTimestamp.
     * @member {number|Long} dataTimestamp
     * @memberof TrainArrivalHistoryEntry
     * @instance
     */
    TrainArrivalHistoryEntry.prototype.dataTimestamp = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * TrainArrivalHistoryEntry arrivalTime.
     * @member {number|Long} arrivalTime
     * @memberof TrainArrivalHistoryEntry
     * @instance
     */
    TrainArrivalHistoryEntry.prototype.arrivalTime = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

    /**
     * Creates a new TrainArrivalHistoryEntry instance using the specified properties.
     * @function create
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {ITrainArrivalHistoryEntry=} [properties] Properties to set
     * @returns {TrainArrivalHistoryEntry} TrainArrivalHistoryEntry instance
     */
    TrainArrivalHistoryEntry.create = function create(properties) {
        return new TrainArrivalHistoryEntry(properties);
    };

    /**
     * Encodes the specified TrainArrivalHistoryEntry message. Does not implicitly {@link TrainArrivalHistoryEntry.verify|verify} messages.
     * @function encode
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {ITrainArrivalHistoryEntry} message TrainArrivalHistoryEntry message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainArrivalHistoryEntry.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            writer.uint32(/* id 1, wireType 0 =*/8).int64(message.dataTimestamp);
        if (message.arrivalTime != null && message.hasOwnProperty("arrivalTime"))
            writer.uint32(/* id 2, wireType 0 =*/16).int64(message.arrivalTime);
        return writer;
    };

    /**
     * Encodes the specified TrainArrivalHistoryEntry message, length delimited. Does not implicitly {@link TrainArrivalHistoryEntry.verify|verify} messages.
     * @function encodeDelimited
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {ITrainArrivalHistoryEntry} message TrainArrivalHistoryEntry message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainArrivalHistoryEntry.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a TrainArrivalHistoryEntry message from the specified reader or buffer.
     * @function decode
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {TrainArrivalHistoryEntry} TrainArrivalHistoryEntry
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainArrivalHistoryEntry.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.TrainArrivalHistoryEntry();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.dataTimestamp = reader.int64();
                break;
            case 2:
                message.arrivalTime = reader.int64();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a TrainArrivalHistoryEntry message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {TrainArrivalHistoryEntry} TrainArrivalHistoryEntry
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainArrivalHistoryEntry.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a TrainArrivalHistoryEntry message.
     * @function verify
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    TrainArrivalHistoryEntry.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            if (!$util.isInteger(message.dataTimestamp) && !(message.dataTimestamp && $util.isInteger(message.dataTimestamp.low) && $util.isInteger(message.dataTimestamp.high)))
                return "dataTimestamp: integer|Long expected";
        if (message.arrivalTime != null && message.hasOwnProperty("arrivalTime"))
            if (!$util.isInteger(message.arrivalTime) && !(message.arrivalTime && $util.isInteger(message.arrivalTime.low) && $util.isInteger(message.arrivalTime.high)))
                return "arrivalTime: integer|Long expected";
        return null;
    };

    /**
     * Creates a TrainArrivalHistoryEntry message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {TrainArrivalHistoryEntry} TrainArrivalHistoryEntry
     */
    TrainArrivalHistoryEntry.fromObject = function fromObject(object) {
        if (object instanceof $root.TrainArrivalHistoryEntry)
            return object;
        var message = new $root.TrainArrivalHistoryEntry();
        if (object.dataTimestamp != null)
            if ($util.Long)
                (message.dataTimestamp = $util.Long.fromValue(object.dataTimestamp)).unsigned = false;
            else if (typeof object.dataTimestamp === "string")
                message.dataTimestamp = parseInt(object.dataTimestamp, 10);
            else if (typeof object.dataTimestamp === "number")
                message.dataTimestamp = object.dataTimestamp;
            else if (typeof object.dataTimestamp === "object")
                message.dataTimestamp = new $util.LongBits(object.dataTimestamp.low >>> 0, object.dataTimestamp.high >>> 0).toNumber();
        if (object.arrivalTime != null)
            if ($util.Long)
                (message.arrivalTime = $util.Long.fromValue(object.arrivalTime)).unsigned = false;
            else if (typeof object.arrivalTime === "string")
                message.arrivalTime = parseInt(object.arrivalTime, 10);
            else if (typeof object.arrivalTime === "number")
                message.arrivalTime = object.arrivalTime;
            else if (typeof object.arrivalTime === "object")
                message.arrivalTime = new $util.LongBits(object.arrivalTime.low >>> 0, object.arrivalTime.high >>> 0).toNumber();
        return message;
    };

    /**
     * Creates a plain object from a TrainArrivalHistoryEntry message. Also converts values to other types if specified.
     * @function toObject
     * @memberof TrainArrivalHistoryEntry
     * @static
     * @param {TrainArrivalHistoryEntry} message TrainArrivalHistoryEntry
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    TrainArrivalHistoryEntry.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.dataTimestamp = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.dataTimestamp = options.longs === String ? "0" : 0;
            if ($util.Long) {
                var long = new $util.Long(0, 0, false);
                object.arrivalTime = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
            } else
                object.arrivalTime = options.longs === String ? "0" : 0;
        }
        if (message.dataTimestamp != null && message.hasOwnProperty("dataTimestamp"))
            if (typeof message.dataTimestamp === "number")
                object.dataTimestamp = options.longs === String ? String(message.dataTimestamp) : message.dataTimestamp;
            else
                object.dataTimestamp = options.longs === String ? $util.Long.prototype.toString.call(message.dataTimestamp) : options.longs === Number ? new $util.LongBits(message.dataTimestamp.low >>> 0, message.dataTimestamp.high >>> 0).toNumber() : message.dataTimestamp;
        if (message.arrivalTime != null && message.hasOwnProperty("arrivalTime"))
            if (typeof message.arrivalTime === "number")
                object.arrivalTime = options.longs === String ? String(message.arrivalTime) : message.arrivalTime;
            else
                object.arrivalTime = options.longs === String ? $util.Long.prototype.toString.call(message.arrivalTime) : options.longs === Number ? new $util.LongBits(message.arrivalTime.low >>> 0, message.arrivalTime.high >>> 0).toNumber() : message.arrivalTime;
        return object;
    };

    /**
     * Converts this TrainArrivalHistoryEntry to JSON.
     * @function toJSON
     * @memberof TrainArrivalHistoryEntry
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    TrainArrivalHistoryEntry.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return TrainArrivalHistoryEntry;
})();

$root.TrainArrivalHistory = (function() {

    /**
     * Properties of a TrainArrivalHistory.
     * @exports ITrainArrivalHistory
     * @interface ITrainArrivalHistory
     * @property {IDebugInfo|null} [debugInfo] TrainArrivalHistory debugInfo
     * @property {Array.<ITrainArrivalHistoryEntry>|null} [history] TrainArrivalHistory history
     */

    /**
     * Constructs a new TrainArrivalHistory.
     * @exports TrainArrivalHistory
     * @classdesc Represents a TrainArrivalHistory.
     * @implements ITrainArrivalHistory
     * @constructor
     * @param {ITrainArrivalHistory=} [properties] Properties to set
     */
    function TrainArrivalHistory(properties) {
        this.history = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * TrainArrivalHistory debugInfo.
     * @member {IDebugInfo|null|undefined} debugInfo
     * @memberof TrainArrivalHistory
     * @instance
     */
    TrainArrivalHistory.prototype.debugInfo = null;

    /**
     * TrainArrivalHistory history.
     * @member {Array.<ITrainArrivalHistoryEntry>} history
     * @memberof TrainArrivalHistory
     * @instance
     */
    TrainArrivalHistory.prototype.history = $util.emptyArray;

    /**
     * Creates a new TrainArrivalHistory instance using the specified properties.
     * @function create
     * @memberof TrainArrivalHistory
     * @static
     * @param {ITrainArrivalHistory=} [properties] Properties to set
     * @returns {TrainArrivalHistory} TrainArrivalHistory instance
     */
    TrainArrivalHistory.create = function create(properties) {
        return new TrainArrivalHistory(properties);
    };

    /**
     * Encodes the specified TrainArrivalHistory message. Does not implicitly {@link TrainArrivalHistory.verify|verify} messages.
     * @function encode
     * @memberof TrainArrivalHistory
     * @static
     * @param {ITrainArrivalHistory} message TrainArrivalHistory message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainArrivalHistory.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            $root.DebugInfo.encode(message.debugInfo, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
        if (message.history != null && message.history.length)
            for (var i = 0; i < message.history.length; ++i)
                $root.TrainArrivalHistoryEntry.encode(message.history[i], writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
        return writer;
    };

    /**
     * Encodes the specified TrainArrivalHistory message, length delimited. Does not implicitly {@link TrainArrivalHistory.verify|verify} messages.
     * @function encodeDelimited
     * @memberof TrainArrivalHistory
     * @static
     * @param {ITrainArrivalHistory} message TrainArrivalHistory message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    TrainArrivalHistory.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a TrainArrivalHistory message from the specified reader or buffer.
     * @function decode
     * @memberof TrainArrivalHistory
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {TrainArrivalHistory} TrainArrivalHistory
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainArrivalHistory.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.TrainArrivalHistory();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.debugInfo = $root.DebugInfo.decode(reader, reader.uint32());
                break;
            case 2:
                if (!(message.history && message.history.length))
                    message.history = [];
                message.history.push($root.TrainArrivalHistoryEntry.decode(reader, reader.uint32()));
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a TrainArrivalHistory message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof TrainArrivalHistory
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {TrainArrivalHistory} TrainArrivalHistory
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    TrainArrivalHistory.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a TrainArrivalHistory message.
     * @function verify
     * @memberof TrainArrivalHistory
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    TrainArrivalHistory.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo")) {
            var error = $root.DebugInfo.verify(message.debugInfo);
            if (error)
                return "debugInfo." + error;
        }
        if (message.history != null && message.hasOwnProperty("history")) {
            if (!Array.isArray(message.history))
                return "history: array expected";
            for (var i = 0; i < message.history.length; ++i) {
                var error = $root.TrainArrivalHistoryEntry.verify(message.history[i]);
                if (error)
                    return "history." + error;
            }
        }
        return null;
    };

    /**
     * Creates a TrainArrivalHistory message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof TrainArrivalHistory
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {TrainArrivalHistory} TrainArrivalHistory
     */
    TrainArrivalHistory.fromObject = function fromObject(object) {
        if (object instanceof $root.TrainArrivalHistory)
            return object;
        var message = new $root.TrainArrivalHistory();
        if (object.debugInfo != null) {
            if (typeof object.debugInfo !== "object")
                throw TypeError(".TrainArrivalHistory.debugInfo: object expected");
            message.debugInfo = $root.DebugInfo.fromObject(object.debugInfo);
        }
        if (object.history) {
            if (!Array.isArray(object.history))
                throw TypeError(".TrainArrivalHistory.history: array expected");
            message.history = [];
            for (var i = 0; i < object.history.length; ++i) {
                if (typeof object.history[i] !== "object")
                    throw TypeError(".TrainArrivalHistory.history: object expected");
                message.history[i] = $root.TrainArrivalHistoryEntry.fromObject(object.history[i]);
            }
        }
        return message;
    };

    /**
     * Creates a plain object from a TrainArrivalHistory message. Also converts values to other types if specified.
     * @function toObject
     * @memberof TrainArrivalHistory
     * @static
     * @param {TrainArrivalHistory} message TrainArrivalHistory
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    TrainArrivalHistory.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.history = [];
        if (options.defaults)
            object.debugInfo = null;
        if (message.debugInfo != null && message.hasOwnProperty("debugInfo"))
            object.debugInfo = $root.DebugInfo.toObject(message.debugInfo, options);
        if (message.history && message.history.length) {
            object.history = [];
            for (var j = 0; j < message.history.length; ++j)
                object.history[j] = $root.TrainArrivalHistoryEntry.toObject(message.history[j], options);
        }
        return object;
    };

    /**
     * Converts this TrainArrivalHistory to JSON.
     * @function toJSON
     * @memberof TrainArrivalHistory
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    TrainArrivalHistory.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return TrainArrivalHistory;
})();

$root.SubwayStatusMessage = (function() {

    /**
     * Properties of a SubwayStatusMessage.
     * @exports ISubwayStatusMessage
     * @interface ISubwayStatusMessage
     * @property {string|null} [summary] SubwayStatusMessage summary
     * @property {string|null} [longDescription] SubwayStatusMessage longDescription
     * @property {Array.<IAffectedLineStatus>|null} [affectedLine] SubwayStatusMessage affectedLine
     * @property {boolean|null} [planned] SubwayStatusMessage planned
     * @property {string|null} [reasonName] SubwayStatusMessage reasonName
     * @property {number|null} [priority] SubwayStatusMessage priority
     */

    /**
     * Constructs a new SubwayStatusMessage.
     * @exports SubwayStatusMessage
     * @classdesc Represents a SubwayStatusMessage.
     * @implements ISubwayStatusMessage
     * @constructor
     * @param {ISubwayStatusMessage=} [properties] Properties to set
     */
    function SubwayStatusMessage(properties) {
        this.affectedLine = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * SubwayStatusMessage summary.
     * @member {string} summary
     * @memberof SubwayStatusMessage
     * @instance
     */
    SubwayStatusMessage.prototype.summary = "";

    /**
     * SubwayStatusMessage longDescription.
     * @member {string} longDescription
     * @memberof SubwayStatusMessage
     * @instance
     */
    SubwayStatusMessage.prototype.longDescription = "";

    /**
     * SubwayStatusMessage affectedLine.
     * @member {Array.<IAffectedLineStatus>} affectedLine
     * @memberof SubwayStatusMessage
     * @instance
     */
    SubwayStatusMessage.prototype.affectedLine = $util.emptyArray;

    /**
     * SubwayStatusMessage planned.
     * @member {boolean} planned
     * @memberof SubwayStatusMessage
     * @instance
     */
    SubwayStatusMessage.prototype.planned = false;

    /**
     * SubwayStatusMessage reasonName.
     * @member {string} reasonName
     * @memberof SubwayStatusMessage
     * @instance
     */
    SubwayStatusMessage.prototype.reasonName = "";

    /**
     * SubwayStatusMessage priority.
     * @member {number} priority
     * @memberof SubwayStatusMessage
     * @instance
     */
    SubwayStatusMessage.prototype.priority = 0;

    /**
     * Creates a new SubwayStatusMessage instance using the specified properties.
     * @function create
     * @memberof SubwayStatusMessage
     * @static
     * @param {ISubwayStatusMessage=} [properties] Properties to set
     * @returns {SubwayStatusMessage} SubwayStatusMessage instance
     */
    SubwayStatusMessage.create = function create(properties) {
        return new SubwayStatusMessage(properties);
    };

    /**
     * Encodes the specified SubwayStatusMessage message. Does not implicitly {@link SubwayStatusMessage.verify|verify} messages.
     * @function encode
     * @memberof SubwayStatusMessage
     * @static
     * @param {ISubwayStatusMessage} message SubwayStatusMessage message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    SubwayStatusMessage.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.summary != null && message.hasOwnProperty("summary"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.summary);
        if (message.longDescription != null && message.hasOwnProperty("longDescription"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.longDescription);
        if (message.affectedLine != null && message.affectedLine.length)
            for (var i = 0; i < message.affectedLine.length; ++i)
                $root.AffectedLineStatus.encode(message.affectedLine[i], writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
        if (message.planned != null && message.hasOwnProperty("planned"))
            writer.uint32(/* id 4, wireType 0 =*/32).bool(message.planned);
        if (message.reasonName != null && message.hasOwnProperty("reasonName"))
            writer.uint32(/* id 5, wireType 2 =*/42).string(message.reasonName);
        if (message.priority != null && message.hasOwnProperty("priority"))
            writer.uint32(/* id 6, wireType 0 =*/48).int32(message.priority);
        return writer;
    };

    /**
     * Encodes the specified SubwayStatusMessage message, length delimited. Does not implicitly {@link SubwayStatusMessage.verify|verify} messages.
     * @function encodeDelimited
     * @memberof SubwayStatusMessage
     * @static
     * @param {ISubwayStatusMessage} message SubwayStatusMessage message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    SubwayStatusMessage.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a SubwayStatusMessage message from the specified reader or buffer.
     * @function decode
     * @memberof SubwayStatusMessage
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {SubwayStatusMessage} SubwayStatusMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    SubwayStatusMessage.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.SubwayStatusMessage();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.summary = reader.string();
                break;
            case 2:
                message.longDescription = reader.string();
                break;
            case 3:
                if (!(message.affectedLine && message.affectedLine.length))
                    message.affectedLine = [];
                message.affectedLine.push($root.AffectedLineStatus.decode(reader, reader.uint32()));
                break;
            case 4:
                message.planned = reader.bool();
                break;
            case 5:
                message.reasonName = reader.string();
                break;
            case 6:
                message.priority = reader.int32();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a SubwayStatusMessage message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof SubwayStatusMessage
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {SubwayStatusMessage} SubwayStatusMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    SubwayStatusMessage.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a SubwayStatusMessage message.
     * @function verify
     * @memberof SubwayStatusMessage
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    SubwayStatusMessage.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.summary != null && message.hasOwnProperty("summary"))
            if (!$util.isString(message.summary))
                return "summary: string expected";
        if (message.longDescription != null && message.hasOwnProperty("longDescription"))
            if (!$util.isString(message.longDescription))
                return "longDescription: string expected";
        if (message.affectedLine != null && message.hasOwnProperty("affectedLine")) {
            if (!Array.isArray(message.affectedLine))
                return "affectedLine: array expected";
            for (var i = 0; i < message.affectedLine.length; ++i) {
                var error = $root.AffectedLineStatus.verify(message.affectedLine[i]);
                if (error)
                    return "affectedLine." + error;
            }
        }
        if (message.planned != null && message.hasOwnProperty("planned"))
            if (typeof message.planned !== "boolean")
                return "planned: boolean expected";
        if (message.reasonName != null && message.hasOwnProperty("reasonName"))
            if (!$util.isString(message.reasonName))
                return "reasonName: string expected";
        if (message.priority != null && message.hasOwnProperty("priority"))
            if (!$util.isInteger(message.priority))
                return "priority: integer expected";
        return null;
    };

    /**
     * Creates a SubwayStatusMessage message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof SubwayStatusMessage
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {SubwayStatusMessage} SubwayStatusMessage
     */
    SubwayStatusMessage.fromObject = function fromObject(object) {
        if (object instanceof $root.SubwayStatusMessage)
            return object;
        var message = new $root.SubwayStatusMessage();
        if (object.summary != null)
            message.summary = String(object.summary);
        if (object.longDescription != null)
            message.longDescription = String(object.longDescription);
        if (object.affectedLine) {
            if (!Array.isArray(object.affectedLine))
                throw TypeError(".SubwayStatusMessage.affectedLine: array expected");
            message.affectedLine = [];
            for (var i = 0; i < object.affectedLine.length; ++i) {
                if (typeof object.affectedLine[i] !== "object")
                    throw TypeError(".SubwayStatusMessage.affectedLine: object expected");
                message.affectedLine[i] = $root.AffectedLineStatus.fromObject(object.affectedLine[i]);
            }
        }
        if (object.planned != null)
            message.planned = Boolean(object.planned);
        if (object.reasonName != null)
            message.reasonName = String(object.reasonName);
        if (object.priority != null)
            message.priority = object.priority | 0;
        return message;
    };

    /**
     * Creates a plain object from a SubwayStatusMessage message. Also converts values to other types if specified.
     * @function toObject
     * @memberof SubwayStatusMessage
     * @static
     * @param {SubwayStatusMessage} message SubwayStatusMessage
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    SubwayStatusMessage.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.affectedLine = [];
        if (options.defaults) {
            object.summary = "";
            object.longDescription = "";
            object.planned = false;
            object.reasonName = "";
            object.priority = 0;
        }
        if (message.summary != null && message.hasOwnProperty("summary"))
            object.summary = message.summary;
        if (message.longDescription != null && message.hasOwnProperty("longDescription"))
            object.longDescription = message.longDescription;
        if (message.affectedLine && message.affectedLine.length) {
            object.affectedLine = [];
            for (var j = 0; j < message.affectedLine.length; ++j)
                object.affectedLine[j] = $root.AffectedLineStatus.toObject(message.affectedLine[j], options);
        }
        if (message.planned != null && message.hasOwnProperty("planned"))
            object.planned = message.planned;
        if (message.reasonName != null && message.hasOwnProperty("reasonName"))
            object.reasonName = message.reasonName;
        if (message.priority != null && message.hasOwnProperty("priority"))
            object.priority = message.priority;
        return object;
    };

    /**
     * Converts this SubwayStatusMessage to JSON.
     * @function toJSON
     * @memberof SubwayStatusMessage
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    SubwayStatusMessage.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return SubwayStatusMessage;
})();

$root.AffectedLineStatus = (function() {

    /**
     * Properties of an AffectedLineStatus.
     * @exports IAffectedLineStatus
     * @interface IAffectedLineStatus
     * @property {string|null} [line] AffectedLineStatus line
     * @property {Direction|null} [direction] AffectedLineStatus direction
     */

    /**
     * Constructs a new AffectedLineStatus.
     * @exports AffectedLineStatus
     * @classdesc Represents an AffectedLineStatus.
     * @implements IAffectedLineStatus
     * @constructor
     * @param {IAffectedLineStatus=} [properties] Properties to set
     */
    function AffectedLineStatus(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * AffectedLineStatus line.
     * @member {string} line
     * @memberof AffectedLineStatus
     * @instance
     */
    AffectedLineStatus.prototype.line = "";

    /**
     * AffectedLineStatus direction.
     * @member {Direction} direction
     * @memberof AffectedLineStatus
     * @instance
     */
    AffectedLineStatus.prototype.direction = 0;

    /**
     * Creates a new AffectedLineStatus instance using the specified properties.
     * @function create
     * @memberof AffectedLineStatus
     * @static
     * @param {IAffectedLineStatus=} [properties] Properties to set
     * @returns {AffectedLineStatus} AffectedLineStatus instance
     */
    AffectedLineStatus.create = function create(properties) {
        return new AffectedLineStatus(properties);
    };

    /**
     * Encodes the specified AffectedLineStatus message. Does not implicitly {@link AffectedLineStatus.verify|verify} messages.
     * @function encode
     * @memberof AffectedLineStatus
     * @static
     * @param {IAffectedLineStatus} message AffectedLineStatus message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    AffectedLineStatus.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.line != null && message.hasOwnProperty("line"))
            writer.uint32(/* id 1, wireType 2 =*/10).string(message.line);
        if (message.direction != null && message.hasOwnProperty("direction"))
            writer.uint32(/* id 2, wireType 0 =*/16).int32(message.direction);
        return writer;
    };

    /**
     * Encodes the specified AffectedLineStatus message, length delimited. Does not implicitly {@link AffectedLineStatus.verify|verify} messages.
     * @function encodeDelimited
     * @memberof AffectedLineStatus
     * @static
     * @param {IAffectedLineStatus} message AffectedLineStatus message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    AffectedLineStatus.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes an AffectedLineStatus message from the specified reader or buffer.
     * @function decode
     * @memberof AffectedLineStatus
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {AffectedLineStatus} AffectedLineStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    AffectedLineStatus.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.AffectedLineStatus();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.line = reader.string();
                break;
            case 2:
                message.direction = reader.int32();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes an AffectedLineStatus message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof AffectedLineStatus
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {AffectedLineStatus} AffectedLineStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    AffectedLineStatus.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies an AffectedLineStatus message.
     * @function verify
     * @memberof AffectedLineStatus
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    AffectedLineStatus.verify = function verify(message) {
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
        return null;
    };

    /**
     * Creates an AffectedLineStatus message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof AffectedLineStatus
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {AffectedLineStatus} AffectedLineStatus
     */
    AffectedLineStatus.fromObject = function fromObject(object) {
        if (object instanceof $root.AffectedLineStatus)
            return object;
        var message = new $root.AffectedLineStatus();
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
        return message;
    };

    /**
     * Creates a plain object from an AffectedLineStatus message. Also converts values to other types if specified.
     * @function toObject
     * @memberof AffectedLineStatus
     * @static
     * @param {AffectedLineStatus} message AffectedLineStatus
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    AffectedLineStatus.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.line = "";
            object.direction = options.enums === String ? "UPTOWN" : 0;
        }
        if (message.line != null && message.hasOwnProperty("line"))
            object.line = message.line;
        if (message.direction != null && message.hasOwnProperty("direction"))
            object.direction = options.enums === String ? $root.Direction[message.direction] : message.direction;
        return object;
    };

    /**
     * Converts this AffectedLineStatus to JSON.
     * @function toJSON
     * @memberof AffectedLineStatus
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    AffectedLineStatus.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return AffectedLineStatus;
})();

module.exports = $root;
