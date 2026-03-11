#include<stdio.h>

int main(int argc, char* argv[])
{
  char c;
  int i, j, h, w;
  scanf("%d %d", &h, &w);

  while((h != 0)||(w != 0)){
    for(i = 0; i < h; i++){
      if((i == 0)||(i == h - 1)){
        c = '#';
      }else{
        c = '.';
      }
      for(j = 0; j < w; j++){
        if((j == 0)||(j == w - 1)){
          printf("#");
        }else{
          printf("%c", c);
        }
      }
      printf("\n");
    }
    printf("\n");
    scanf("%d %d", &h, &w);
  }
  return(0);
}