import * as Cookie from "es-cookie";

const RECENT_STATION_COUNT = 10;
const FREQUENT_STATION_COUNT = 20;  // Keep stats for this many frequently-used stations
const RECENT_STATION_GRACE_WINDOW = 5;  // This many recently used stations can't be evicted from the frequency map

export class StationStats {
  recentStations: string[];  // Most recent first
  frequentStations: Map<string, number>;  // Station ID -> usage count

  constructor(recentStations: string[], frequentStations: Map<string, number>) {
    this.recentStations = recentStations;
    this.frequentStations = frequentStations;
  }

  static empty(): StationStats {
    return new StationStats([], new Map());
  }

  recordStationAccess(stationId: string) {
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

  serialize(): string {
    return JSON.stringify(this, replacer);
  }

  static deserialize(serialized: string): StationStats {
    let data = JSON.parse(serialized, reviver);
    return new StationStats(data.recentStations, data.frequentStations);
  }

  static fromCookie(): StationStats {
    const serializedStationStats = Cookie.get("stationStats");
    console.log("got cookie: " + serializedStationStats);

    if (serializedStationStats) {
      return StationStats.deserialize(serializedStationStats);
    } else {
      return StationStats.empty();
    }
  }

  saveToCookie() {
    const serializedStationStats = this.serialize();
    console.log("SetCookie('stationStats', '" + serializedStationStats + "')");
    Cookie.set("stationStats", serializedStationStats);
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
export function recentStationsFromCookie(): string[] {
  const recentStationsStr = Cookie.get("recentStations");

  if (recentStationsStr) {
    return recentStationsStr.split(":").reverse();
  } else {
    return [];
  }
}
