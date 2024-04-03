const std = @import("std");

pub fn isArmstrongNumber(num: u128) bool {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = general_purpose_allocator.allocator();
    const num_str: []u8 = std.fmt.allocPrint(allocator, "{d}", .{num}) catch unreachable;
    const length = num_str.len;

    var sum: u128 = 0;
    for (num_str) |c| {
        const digit = c - '0';
        sum += std.math.powi(u128, digit, length) catch unreachable;
    }

    return num == sum;
}
