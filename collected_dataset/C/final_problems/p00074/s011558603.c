#include <stdio.h>

void timePrint(int time);

int main(void){
  int t, h, s;
  int time;

  scanf("%d%d%d", &t, &h, &s);
  while(!(t == -1 && h == -1 && s == -1)){
    time = t*3600 + h*60 + s;
    timePrint(7200-time);
    timePrint(7200-time/3);
    scanf("%d%d%d", &t, &h, &s);
  }
  return 0;
}

void timePrint(int time){
  printf("%02d:", time/3600);
  time %= 3600;
  printf("%02d:", time/60);
  time %= 60;
  printf("%02d\n", time);
  return;
}