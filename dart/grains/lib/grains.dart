BigInt square(final int n) { 
    if (n <= 0 || n > 64) {
        throw ArgumentError('square must be between 1 and 64');
    }
    return BigInt.from(2).pow(n - 1);
}

BigInt total() {
  BigInt total = BigInt.from(0);
  
  for (var i = 1; i <= 64; i++) {
    total += square(i);
  }
  
  return total;
}
