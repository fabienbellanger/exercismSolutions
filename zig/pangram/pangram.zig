const std = @import("std");
const ascii = std.ascii;

pub fn isPangram(str: []const u8) bool {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var set = std.AutoHashMap(u8, void).init(allocator);
    defer set.deinit();

    for (str) |c| {
        if (ascii.isAlphabetic(c)) {
            set.put(ascii.toLower(c), {}) catch unreachable;
        }
    }

    return set.count() == 26;
}
