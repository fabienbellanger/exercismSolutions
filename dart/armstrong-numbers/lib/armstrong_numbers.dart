import 'dart:math';

class ArmstrongNumbers {
  bool isArmstrongNumber(int n) {
    var numbers =
        n.toString().split('').map((c) => int.parse(c));

    return numbers.fold(
            0, (p, e) => (p as int) + pow(e, numbers.length) as int) ==
        n;
  }
}
