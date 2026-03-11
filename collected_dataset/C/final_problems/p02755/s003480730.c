#include <stdio.h>

int main(void){

  float a,b;
  scanf("%f %f",&a,&b);

  float kagen = b/0.1;
  float jogen = (b+1.0)/0.1;

  int answer = -1;
  int start = kagen/1;
  if (start*10/100 != b) {
    start++;
  }
  for (int i = start; i < jogen; i++) {
    if (i*8/100 == a) {
      answer = i;
      break;
    }
  }

  printf("%d\n", answer);

  return 0;
}