
import { BigDecimal } from "@graphprotocol/graph-ts";
import { Timestamp } from "./pb/google/protobuf/Timestamp";


export function addAssets(quantity1: string, quantity2: string): string {
    const asset1 = quantity1.split(" ");
    const asset2 = quantity2.split(" ");
    if (asset1[1] != asset2[1]) {
        throw `symbols mismach: ${asset1[1]} != ${asset2[1]}`;
    }
    const sum = parseFloat(asset1[0]) + parseFloat(asset2[0]);
    return `${toFixed(sum, 4)} ${asset1[1]}`;

}

export function toValue(quantity: string): BigDecimal {
    const asset = quantity.split(" ");
    return BigDecimal.fromString(asset[0]);
}

export function toBigDecimal(num: f64): BigDecimal {
    let numStr = toFixed(num, 18); // Adjust precision as needed
    return BigDecimal.fromString(numStr);
}

export function toFixed(num: f64, decimals: i32): string {
    let factor = Math.pow(10, decimals) as f64;
    let rounded = Math.round(num * factor) / factor;
    let result = rounded.toString();
    let dotIndex = result.indexOf(".");
    if (dotIndex == -1) {
        result += ".";
        dotIndex = result.length - 1;
    }
    let decimalPartLength = result.length - dotIndex - 1;
    for (let i = 0; i < decimals - decimalPartLength; i++) {
        result += "0";
    }
    return result;
}

export function timestampToString(timestamp: Timestamp): string {
    const milliseconds = timestamp.seconds * 1000 + timestamp.nanos / 1000000;
    const date = new Date(milliseconds);
    return date.toISOString();
}