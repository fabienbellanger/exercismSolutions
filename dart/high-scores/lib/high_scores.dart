class HighScores {
  List<int> scores;

  HighScores(this.scores);

  int latest() {
    return scores.last;
  }

  int personalBest() {
    return scores.reduce((value, element) => value > element ? value : element);
  }

  List<int> personalTopThree() {
    final sortedScores = List<int>.from(scores)..sort((a, b) => b.compareTo(a));
    return sortedScores.take(3).toList();
  }
}
