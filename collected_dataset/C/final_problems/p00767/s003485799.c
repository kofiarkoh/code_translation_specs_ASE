#include <stdio.h>

int main(){
  int w, h;
  while(1){
    scanf("%d %d", &h, &w);
    if(!h)break;
    int sq = w*w+h*h;
    h++;
    while(1){
      for(w=h+1;sq>(w*w+h*h);w++);

      if(sq==(w*w+h*h)){
        printf("%d %d\n", h, w);
        break;
      }
      if(h+1==w){
        sq++;
        h=1;
      }
      else h++;
    }
  }
  return 0;
}