#include <stdio.h>

int main()
{
  int w;
  while(scanf("%d", &w), w != -1){
    int bill = 0;
    if(w < 10){
      bill = 1150;
    }else if(w < 20){
      bill = 1150 + (w-10) * 125;
    }else if(w < 30){
      bill = 2400 + (w-20) * 140;
    }else{
      bill = 3800 + (w-30) * 160;
    }
    printf("%d\n", 4280-bill);
  }
  return 0;
}