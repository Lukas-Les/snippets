void setup() {
  pinMode(3, OUTPUT);
  pinMode(4, OUTPUT);
  pinMode(5, OUTPUT);
  pinMode(2, INPUT);
}

void loop() {
  int swichState = 0;
  swichState = digitalRead(2);

  if (swichState == LOW) {
    digitalWrite(3, HIGH);
    digitalWrite(4, LOW);
    digitalWrite(5, LOW);
  } else {
    digitalWrite(3, LOW);
    digitalWrite(4, HIGH);
    digitalWrite(5, LOW);
  
    delay(1000);

    digitalWrite(4, LOW);
    digitalWrite(5, HIGH);
    delay(1000);

    digitalWrite(4, HIGH);
    digitalWrite(5, LOW);
    delay(1000);
  }
}
