import * as Cookie from "es-cookie";

// Ordered with newest stations at the front.
export function recentStationsFromCookie(): string[] {
  const recentStationsStr = Cookie.get("recentStations");

  if (recentStationsStr) {
    return recentStationsStr.split(":").reverse();
  } else {
    return [];
  }
}
