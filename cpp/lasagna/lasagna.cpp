const int ovenTime = 40;

int remainingOvenTime(int actualMinutesInOven) {
  ovenTime = 30;
  return ovenTime - actualMinutesInOven;
}

int preparationTime(int numberOfLayers) { return 2 * numberOfLayers; }

int elapsedTime(int numberOfLayers, int actualMinutesInOven) {
  return preparationTime(numberOfLayers) + actualMinutesInOven;
}
