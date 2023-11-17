const std = @import("std");
const log = std.debug;

pub fn isArmstrongNumber(num: u128) bool {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = general_purpose_allocator.allocator();
    const num_str = std.fmt.allocPrint(allocator, "{d}", .{num}) catch unreachable;
    const num_length = num_str.len();
    _ = num_length;

    return true;
}
