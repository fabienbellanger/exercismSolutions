const std = @import("std");

pub fn isArmstrongNumber(num: u128) bool {
    var length: u128 = 1;
    var n = num;
    while (n >= 10) : (n /= 10) length += 1;

    var sum: u128 = 0;
    n = num;
    while (n != 0) : (n /= 10) {
        sum += std.math.powi(u128, n % 10, length) catch unreachable;
    }

    return num == sum;
}
