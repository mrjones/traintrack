declare namespace proto {
    export class LineArrivals {
        constructor();
        public setLine(line: string): void;
    }

    namespace LineArrivals {
        export function deserializeBinary(b: string): LineArrivals;
    }
}
