const std = @import("std");
const ascii = std.ascii;

pub fn isPangram(str: []const u8) bool {
    var debug_allocator = std.heap.DebugAllocator(.{}){};
    const allocator = debug_allocator.allocator();

    var set = std.AutoHashMap(u8, void).init(allocator);
    defer set.deinit();

    for (str) |c| {
        if (ascii.isAlphabetic(c)) {
            set.put(ascii.toLower(c), {}) catch unreachable;
        }
    }

    return set.count() == 26;
}
