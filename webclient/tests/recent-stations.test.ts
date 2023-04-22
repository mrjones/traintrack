import { StationStats } from '../src/recent-stations'

describe("StationStats", () => {
  it("should keep track of 10 recent stations, in order", () => {
    let stats = StationStats.empty();

    stats.recordStationAccess("1");
    expect(stats.recentStations).toStrictEqual(["1"]);

    stats.recordStationAccess("2");
    expect(stats.recentStations).toStrictEqual(["2", "1"]);  // Most recent first

    stats.recordStationAccess("1");
    expect(stats.recentStations).toStrictEqual(["1", "2"]);  // Most recent first

    for (let i = 3; i <= 11; i++) {
      stats.recordStationAccess("" + i);
    }
    expect(stats.recentStations).toStrictEqual(["11", "10", "9", "8", "7", "6", "5", "4", "3", "1"]);  // only keeps 10
  });

  it("should perfectly keep track of <= 10 frequent stations", () => {
    let stats = StationStats.empty();

    stats.recordStationAccess("1");
    expect(stats.frequentStations).toEqual(new Map<string, number>([["1", 1]]));

    stats.recordStationAccess("1");
    expect(stats.frequentStations).toEqual(new Map<string, number>([["1", 2]]));

    for (let i = 0; i < 98; i++) {
      stats.recordStationAccess("1");
    }
    expect(stats.frequentStations).toEqual(new Map<string, number>([["1", 100]]));

    for (let i = 0; i < 4; i++) {
      stats.recordStationAccess("2");
    }
    expect(stats.frequentStations).toEqual(new Map<string, number>([["1", 100], ["2", 4]]));

    // Keeps perfect status until 10 elements
    for (let i = 3; i <= 10; i++) {
      stats.recordStationAccess("" + i);
      stats.recordStationAccess("" + i);
    }
    expect(stats.frequentStations).toEqual(new Map<string, number>([
      ["1", 100], ["2", 4], ["3", 2], ["4", 2], ["5", 2],
      ["6", 2], ["7", 2], ["8", 2], ["9", 2], ["10", 2] ]));
  });

  it("should allow new stations to enter, even when full", () => {
    let stats = StationStats.empty();

    for (let i = 1; i <= 10; i++) {
      stats.recordStationAccess("" + i);
      stats.recordStationAccess("" + i);
    }
    expect(stats.frequentStations).toEqual(new Map<string, number>([
      ["1", 2], ["2", 2], ["3", 2], ["4", 2], ["5", 2],
      ["6", 2], ["7", 2], ["8", 2], ["9", 2], ["10", 2]]));

    // Now alternate between two new stations and ensure they can make it into the cache
    // Naive least-frequently used would keep evicting one for the other, and neither would
    // enter the stats.
    for (let i = 0; i < 10; i++) {
      stats.recordStationAccess("A");
      stats.recordStationAccess("B");
    }

    expect(stats.frequentStations).toContainEqual(["A", 10]);
    expect(stats.frequentStations).toContainEqual(["B", 10]);
  });

  it("should be serialize+deserialize to the same thing", () => {
    let stats = StationStats.empty();

    for (let i = 1; i <= 10; i++) {
      stats.recordStationAccess("" + i);
      stats.recordStationAccess("" + i);
    }
    expect(stats.frequentStations).toEqual(new Map<string, number>([
      ["1", 2], ["2", 2], ["3", 2], ["4", 2], ["5", 2],
      ["6", 2], ["7", 2], ["8", 2], ["9", 2], ["10", 2]]));
    expect(stats.recentStations).toEqual(
      ["10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]);

    const serialized = stats.serialize();
    console.log("SERIALIZED: " + serialized);
     const copy = StationStats.deserialize(serialized);
     expect(copy).toEqual(stats);
    expect(copy.frequentStations).toEqual(new Map<string, number>([
      ["1", 2], ["2", 2], ["3", 2], ["4", 2], ["5", 2],
      ["6", 2], ["7", 2], ["8", 2], ["9", 2], ["10", 2]]));
    expect(copy.recentStations).toEqual(
      ["10", "9", "8", "7", "6", "5", "4", "3", "2", "1"]);
  });
});
