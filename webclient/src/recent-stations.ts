import * as Cookie from "es-cookie";

const RECENT_STATION_COUNT = 10;
const FREQUENT_STATION_COUNT = 20;  // Keep stats for this many frequently-used stations
const RECENT_STATION_GRACE_WINDOW = 5;  // This many recently used stations can't be evicted from the frequency map

export class StationStats {
  recentStations: string[];  // Most recent first
  frequentStations: Map<string, number>;  // Station ID -> usage count
  lastUpdateTimestamp: number;

  constructor(recentStations: string[], frequentStations: Map<string, number>, lastUpdateTimestamp: number) {
    this.recentStations = recentStations;
    this.frequentStations = frequentStations;
    this.lastUpdateTimestamp = lastUpdateTimestamp;
  }

  static empty(): StationStats {
    return new StationStats([], new Map(), 0);
  }

  recordStationAccess(stationId: string, nowMs: number) {
    // Suppress duplicates within a short window: It's probably the user editing the
    // UI controls, and not navigating to the same station again.
    const SUPPRESS_WINDOW_MS = 60 * 1000;
    if (stationId == this.recentStations[0] &&
        (nowMs - this.lastUpdateTimestamp < SUPPRESS_WINDOW_MS)) {
      this.lastUpdateTimestamp = nowMs;
      return;
    }
    this.lastUpdateTimestamp = nowMs;

    this.recentStations = this.recentStations.filter((x: string) => x != stationId);
    this.recentStations.unshift(stationId);
    this.recentStations = this.recentStations.slice(0, RECENT_STATION_COUNT);

    if (this.frequentStations.has(stationId)) {
      // Update existing entry
      this.frequentStations.set(stationId, this.frequentStations.get(stationId) + 1);
    } else {
      if (this.frequentStations.size >= FREQUENT_STATION_COUNT) {
        // Need to evict someone to make room
        let victimStationId = "";
        let victimCount = -1;

        // Don't evict very-recently accessed stations
        let safeFromEviction = this.recentStations.slice(0, RECENT_STATION_GRACE_WINDOW);

        this.frequentStations.forEach((count: number, id: string) => {
          if ((victimCount == -1 || (count < victimCount)) && !safeFromEviction.includes(id)) {
            victimStationId = id;
            victimCount = count;
          }
        });

        this.frequentStations.delete(victimStationId);
      }

      // Insert a new entry, with count 1
      this.frequentStations.set(stationId, 1);
    }
  }

  rankStations(): string[] {
    // For now, we rank solely on most-frequent.
    // TODO(mrjones): It would be nice to include most-recent somehow as well.
    class IdAndFreq {
      id: string;
      freq: number;

      constructor(id: string, freq: number) {
        this.id = id;
        this.freq = freq;
      }
    }
    let idsAndFreqs: IdAndFreq[] = [];

    this.frequentStations.forEach((freq: number, id: string) => {
      idsAndFreqs.push(new IdAndFreq(id, freq));
    })
    idsAndFreqs = idsAndFreqs.sort((x: IdAndFreq, y: IdAndFreq) => {
      if (x.freq < y.freq) return 1;
      if (x.freq > y.freq) return -1;
      return 0;
    });
    return idsAndFreqs.map((x: IdAndFreq) => { return x.id; });
  }

  serialize(): string {
    return JSON.stringify(this, replacer);
  }

  static deserialize(serialized: string): StationStats {
    let data = JSON.parse(serialized, reviver);
    return new StationStats(data.recentStations, data.frequentStations, data.lastUpdateTimestamp);
  }

  static fromCookie(): StationStats {
    const serializedStationStats = Cookie.get("stationStats");

    if (serializedStationStats) {
      return StationStats.deserialize(serializedStationStats);
    } else {
      return StationStats.empty();
    }
  }

  saveToCookie() {
    const serializedStationStats = this.serialize();
    console.log("SetCookie('stationStats', '" + serializedStationStats + "')");
    Cookie.set("stationStats", serializedStationStats, {expires: 365 * 2});
  }
};

function replacer(key: string, value: any) {
  if(value instanceof Map) {
    return {
      dataType: 'Map',
      value: Array.from(value.entries()), // or with spread: value: [...value]
    };
  } else {
    return value;
  }
}

function reviver(key: string, value: any) {
  if(typeof value === 'object' && value !== null) {
    if (value.dataType === 'Map') {
      return new Map(value.value);
    }
  }
  return value;
}


// Ordered with newest stations at the front.
function recentStationsFromCookie(): string[] {
  const recentStationsStr = Cookie.get("recentStations");

  if (recentStationsStr) {
    return recentStationsStr.split(":").reverse();
  } else {
    return [];
  }
}
