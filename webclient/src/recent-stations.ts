import * as Cookie from "es-cookie";

const RECENT_STATION_COUNT = 10;
const FREQUENT_STATION_COUNT = 10;

export class StationStats {
  recentStations: string[];  // Most recent first
  frequentStations: Map<string, number>;  // Station ID -> usage count

  constructor() {
    this.recentStations = [];
    this.frequentStations = new Map();
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
        let safeFromEviction = this.recentStations.slice(0, 4);

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
};

export function stationStatsFromCookie(): StationStats {
  const stationStatsJson = Cookie.get("stationStats");

  if (stationStatsJson) {
    return JSON.parse(stationStatsJson);
  } else {
    return new StationStats();
  }
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
