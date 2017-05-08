declare namespace proto {
    export enum Direction {
        UPTOWN,
        DOWNTOWN,
    }

    export class LineArrivals {
        constructor();

        public getLine(): string;
        public setLine(line: string): void;

        public addTimestamp(ts: number): void;
        public setTimestampList(tss: number[]): void;
        public getTimestampList(): number[];

        public getDirection(): Direction;
        public setDirection(d: Direction): void;
    }

    export class StationStatus {
        constructor();

        public getName(): string;
        public setName(name: string): void;

        public addLine(line: LineArrivals): void;
        public setLineList(lines: LineArrivals[]): void;
        public getTimestampList(): LineArrivals[];
    }

    namespace LineArrivals {
        export function deserializeBinary(b: string): LineArrivals;
    }

    namespace StationStatus {
        export function deserializeBinary(b: string): LineArrivals;
    }
}
