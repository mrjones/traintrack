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
 * @enum {number}
 * @property {number} UPTOWN=0 UPTOWN value
 * @property {number} DOWNTOWN=1 DOWNTOWN value
 */
$root.Direction = (function() {
    var valuesById = {}, values = Object.create(valuesById);
    values[valuesById[0] = "UPTOWN"] = 0;
    values[valuesById[1] = "DOWNTOWN"] = 1;
    return values;
})();

$root.LineArrivals = (function() {

    /**
     * Properties of a LineArrivals.
     * @exports ILineArrivals
     * @interface ILineArrivals
     * @property {string} [line] LineArrivals line
     * @property {Direction} [direction] LineArrivals direction
     * @property {Array.<number|Long>} [timestamp] LineArrivals timestamp
     */

    /**
     * Constructs a new LineArrivals.
     * @exports LineArrivals
     * @classdesc Represents a LineArrivals.
     * @constructor
     * @param {ILineArrivals=} [properties] Properties to set
     */
    function LineArrivals(properties) {
        this.timestamp = [];
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * LineArrivals line.
     * @type {string}
     */
    LineArrivals.prototype.line = "";

    /**
     * LineArrivals direction.
     * @type {Direction}
     */
    LineArrivals.prototype.direction = 0;

    /**
     * LineArrivals timestamp.
     * @type {Array.<number|Long>}
     */
    LineArrivals.prototype.timestamp = $util.emptyArray;

    /**
     * Creates a new LineArrivals instance using the specified properties.
     * @param {ILineArrivals=} [properties] Properties to set
     * @returns {LineArrivals} LineArrivals instance
     */
    LineArrivals.create = function create(properties) {
        return new LineArrivals(properties);
    };

    /**
     * Encodes the specified LineArrivals message. Does not implicitly {@link LineArrivals.verify|verify} messages.
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
        if (message.timestamp != null && message.timestamp.length)
            for (var i = 0; i < message.timestamp.length; ++i)
                writer.uint32(/* id 3, wireType 0 =*/24).int64(message.timestamp[i]);
        return writer;
    };

    /**
     * Encodes the specified LineArrivals message, length delimited. Does not implicitly {@link LineArrivals.verify|verify} messages.
     * @param {ILineArrivals} message LineArrivals message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    LineArrivals.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a LineArrivals message from the specified reader or buffer.
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
            case 3:
                if (!(message.timestamp && message.timestamp.length))
                    message.timestamp = [];
                if ((tag & 7) === 2) {
                    var end2 = reader.uint32() + reader.pos;
                    while (reader.pos < end2)
                        message.timestamp.push(reader.int64());
                } else
                    message.timestamp.push(reader.int64());
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
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {LineArrivals} LineArrivals
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    LineArrivals.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a LineArrivals message.
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
        if (message.timestamp != null && message.hasOwnProperty("timestamp")) {
            if (!Array.isArray(message.timestamp))
                return "timestamp: array expected";
            for (var i = 0; i < message.timestamp.length; ++i)
                if (!$util.isInteger(message.timestamp[i]) && !(message.timestamp[i] && $util.isInteger(message.timestamp[i].low) && $util.isInteger(message.timestamp[i].high)))
                    return "timestamp: integer|Long[] expected";
        }
        return null;
    };

    /**
     * Creates a LineArrivals message from a plain object. Also converts values to their respective internal types.
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
        if (object.timestamp) {
            if (!Array.isArray(object.timestamp))
                throw TypeError(".LineArrivals.timestamp: array expected");
            message.timestamp = [];
            for (var i = 0; i < object.timestamp.length; ++i)
                if ($util.Long)
                    (message.timestamp[i] = $util.Long.fromValue(object.timestamp[i])).unsigned = false;
                else if (typeof object.timestamp[i] === "string")
                    message.timestamp[i] = parseInt(object.timestamp[i], 10);
                else if (typeof object.timestamp[i] === "number")
                    message.timestamp[i] = object.timestamp[i];
                else if (typeof object.timestamp[i] === "object")
                    message.timestamp[i] = new $util.LongBits(object.timestamp[i].low >>> 0, object.timestamp[i].high >>> 0).toNumber();
        }
        return message;
    };

    /**
     * Creates a plain object from a LineArrivals message. Also converts values to other types if specified.
     * @param {LineArrivals} message LineArrivals
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    LineArrivals.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.arrays || options.defaults)
            object.timestamp = [];
        if (options.defaults) {
            object.line = "";
            object.direction = options.enums === String ? "UPTOWN" : 0;
        }
        if (message.line != null && message.hasOwnProperty("line"))
            object.line = message.line;
        if (message.direction != null && message.hasOwnProperty("direction"))
            object.direction = options.enums === String ? $root.Direction[message.direction] : message.direction;
        if (message.timestamp && message.timestamp.length) {
            object.timestamp = [];
            for (var j = 0; j < message.timestamp.length; ++j)
                if (typeof message.timestamp[j] === "number")
                    object.timestamp[j] = options.longs === String ? String(message.timestamp[j]) : message.timestamp[j];
                else
                    object.timestamp[j] = options.longs === String ? $util.Long.prototype.toString.call(message.timestamp[j]) : options.longs === Number ? new $util.LongBits(message.timestamp[j].low >>> 0, message.timestamp[j].high >>> 0).toNumber() : message.timestamp[j];
        }
        return object;
    };

    /**
     * Converts this LineArrivals to JSON.
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
     * @type {string}
     */
    StationStatus.prototype.name = "";

    /**
     * StationStatus line.
     * @type {Array.<ILineArrivals>}
     */
    StationStatus.prototype.line = $util.emptyArray;

    /**
     * Creates a new StationStatus instance using the specified properties.
     * @param {IStationStatus=} [properties] Properties to set
     * @returns {StationStatus} StationStatus instance
     */
    StationStatus.create = function create(properties) {
        return new StationStatus(properties);
    };

    /**
     * Encodes the specified StationStatus message. Does not implicitly {@link StationStatus.verify|verify} messages.
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
        return writer;
    };

    /**
     * Encodes the specified StationStatus message, length delimited. Does not implicitly {@link StationStatus.verify|verify} messages.
     * @param {IStationStatus} message StationStatus message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    StationStatus.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a StationStatus message from the specified reader or buffer.
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
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a StationStatus message from the specified reader or buffer, length delimited.
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {StationStatus} StationStatus
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    StationStatus.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a StationStatus message.
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
        return null;
    };

    /**
     * Creates a StationStatus message from a plain object. Also converts values to their respective internal types.
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
        return message;
    };

    /**
     * Creates a plain object from a StationStatus message. Also converts values to other types if specified.
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
        if (options.defaults)
            object.name = "";
        if (message.name != null && message.hasOwnProperty("name"))
            object.name = message.name;
        if (message.line && message.line.length) {
            object.line = [];
            for (var j = 0; j < message.line.length; ++j)
                object.line[j] = $root.LineArrivals.toObject(message.line[j], options);
        }
        return object;
    };

    /**
     * Converts this StationStatus to JSON.
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
     * @type {string}
     */
    Station.prototype.id = "";

    /**
     * Station name.
     * @type {string}
     */
    Station.prototype.name = "";

    /**
     * Creates a new Station instance using the specified properties.
     * @param {IStation=} [properties] Properties to set
     * @returns {Station} Station instance
     */
    Station.create = function create(properties) {
        return new Station(properties);
    };

    /**
     * Encodes the specified Station message. Does not implicitly {@link Station.verify|verify} messages.
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
     * @param {IStation} message Station message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    Station.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a Station message from the specified reader or buffer.
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
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {Station} Station
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    Station.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a Station message.
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
     * @type {Array.<IStation>}
     */
    StationList.prototype.station = $util.emptyArray;

    /**
     * Creates a new StationList instance using the specified properties.
     * @param {IStationList=} [properties] Properties to set
     * @returns {StationList} StationList instance
     */
    StationList.create = function create(properties) {
        return new StationList(properties);
    };

    /**
     * Encodes the specified StationList message. Does not implicitly {@link StationList.verify|verify} messages.
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
        return writer;
    };

    /**
     * Encodes the specified StationList message, length delimited. Does not implicitly {@link StationList.verify|verify} messages.
     * @param {IStationList} message StationList message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    StationList.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a StationList message from the specified reader or buffer.
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
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a StationList message from the specified reader or buffer, length delimited.
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {StationList} StationList
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    StationList.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a StationList message.
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
        return null;
    };

    /**
     * Creates a StationList message from a plain object. Also converts values to their respective internal types.
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
        return message;
    };

    /**
     * Creates a plain object from a StationList message. Also converts values to other types if specified.
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
        if (message.station && message.station.length) {
            object.station = [];
            for (var j = 0; j < message.station.length; ++j)
                object.station[j] = $root.Station.toObject(message.station[j], options);
        }
        return object;
    };

    /**
     * Converts this StationList to JSON.
     * @returns {Object.<string,*>} JSON object
     */
    StationList.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return StationList;
})();

module.exports = $root;
