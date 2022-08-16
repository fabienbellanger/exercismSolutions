import 'dart:math';

class ArmstrongNumbers {
  bool isArmstrongNumber(int n) {
    var numbers =
        n.toString().runes.map((c) => int.parse(String.fromCharCode(c)));

    return numbers.fold(
            0, (p, e) => (p as int) + pow(e, numbers.length) as int) ==
        n;
  }
}
