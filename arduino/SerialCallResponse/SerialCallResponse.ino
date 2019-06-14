
#define PIN_NUM 10
int pins[PIN_NUM] = {2,3,4,5,6,7,8,9,10,11};
int inByte;         // incoming serial byte

void setup() {
  // start serial port at 9600 bps:
  Serial.begin(9600);
  while (!Serial) {
    ; // wait for serial port to connect. Needed for native USB port only
  }
}


void loop() {
  // if we get a valid byte, read analog ins:
  if (Serial.available() > 0) {
    // get incÚoming byte:
    int incomingData = Serial.parseInt();
    if (incomingData <= PIN_NUM && incomingData >= 0) {
      for (int i = 0; i < incomingData; i++) {
        analogWrite(pins[i], 255);
      }
      for (int i = incomingData; i < PIN_NUM; i++) {
        analogWrite(pins[i], 0);
      }
    }
  }
/*
    if (incomingData <= 256 || incomingData >= 0) {
      float pfloat = ((float) incomingData) / 255.0f;
      float output = falloff(output, pfloat);
      int numPins = (int) 6.0f * output;
      
      for (int i = 0; i < numPins; i++) {
        analogWrite(pins[i], 255);
      }
      analogWrite(pins[numPins], incomingData);
      for (int i = numPins + 1; i < 6; i++) {
        analogWrite(pins[i], 0);
      }
    }
  }
*/
}
