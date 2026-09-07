function police(pin1 int, pin2 int) {
  digital_write("A", pin1, true);
  delay(100);

  digital_write("A", pin2, true);
  delay(100);

  digital_write("A", pin1, false);
  delay(100);

  digital_write("A", pin2, false);
  delay(100);
}

let a = 0;

while true {
  police(5, 6);

  a++;
}
