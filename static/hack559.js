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

var draw = function(data, direction) {
  let width = 600;

  let cols;
  if (direction == "DOWNTOWN") {
    cols = {
      "R20": 50 + 0 * ((width - 100) / 3),
      "R30": 50 + 1 * ((width - 100) / 3),
      "R31": 50 + 2 * ((width - 100) / 3),
      "R32": 50 + 3 * ((width - 100) / 3),
    };
  } else {
    cols = {
      "R32": 50 + 0 * ((width - 100) / 3),
      "R31": 50 + 1 * ((width - 100) / 3),
      "R30": 50 + 2 * ((width - 100) / 3),
      "R20": 50 + 3 * ((width - 100) / 3),
    };
  }

  let xPoint = function(point) {
    console.log("PLOTTING: " + JSON.stringify(point));
    return cols[point[0]];
  };

  let scaleDuration = function(duration_seconds) {
    return duration_seconds / 3;
  };

  let yPoint = function(point) {
    return scaleDuration(point[1] - (Date.now() / 1000));
  };

  let lineSpec = d3.line()
                   .x(xPoint)
                   .y(yPoint);

  console.log("Will draw: " + JSON.stringify(data[0].stops));

  let svg = d3.select("body")
              .append("svg")
              .attr("width", width)
              .attr("height", 1000);

  {
    let legendSpec = d3.line()
                       .x(width - 25)
                       .y(scaleDuration);

    for (var i = 0; i < 10; i++) {
      svg.append("rect")
         .attr("fill", (i % 2 == 0) ? "#EFEFEF" : "#FFFFFF")
         .attr("width", width)
         .attr("height", scaleDuration(5 * 60))
         .attr("x", 0)
         .attr("y", scaleDuration(5 * 60 * i));

      svg.append("text")
         .attr("x", width - 20)
         .attr("y", scaleDuration(5 * 60 * i) + 5)
         .text(5 * i);
      svg.append("text")
         .attr("x", 5)
         .attr("y", scaleDuration(5 * 60 * i) + 5)
         .text(5 * i);
    }
  }

  for (var i = 0; i < data.length; i++) {
    if (data[i].direction == direction && data[i].route_id != "W") {
      svg.append("path")
         .datum(data[i].stops)
         .attr("stroke", "steelblue")
         .attr("stroke-width", 1.5)
         .attr("fill", "none")
         .attr('d', lineSpec);

      if (data[i].stops.length > 0) {
        let firstPoint = data[i].stops[0];
        svg.append('text')
           .attr('x', xPoint(firstPoint) - 15)
           .attr('y', yPoint(firstPoint))
           .text(data[i].route_id);
      }
    }
  }

  console.log("finished");
}
