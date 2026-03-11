
#include<stdio.h>

int main(void){
  int W,H;
  char c;

  scanf("%d %d %c",&W,&H,&c);
  for(int h=0;h<H;h++){
    for(int w=0;w<W;w++){
      if(h==0 || h==H-1){
        if(w==0) printf("+");
        else if(w==W-1) printf("+\n");
        else printf("-");
      }else if(h==H/2 && w==W/2){
          printf("%c",c);
      }else{
        if(w==0) printf("|");
        else if(w==W-1) printf("|\n");
        else printf(".");
      }
    }
  }
  return 0;
}

