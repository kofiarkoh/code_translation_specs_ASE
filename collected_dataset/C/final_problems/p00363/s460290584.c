#include<stdio.h>
#include<math.h>
int main(){
  int w,h;
  char a;
  char c[25][25];

  scanf("%d %d %c",&h,&w,&a);
  for(int i=0; i<w; i++){
    for(int j=0; j<h; j++){
      if(i==0 || i==w-1){
        if(j==0 || j==h-1) c[i][j] = '+';
        else c[i][j] = '-';
      }
      else{
        if(j==0 || j==h-1) c[i][j] = '|';
        else c[i][j] = '.';
      }
    }
  }
  c[w/2][h/2] = a;
  for(int i=0; i<w; i++){
    for(int j=0; j<h; j++){
      printf("%c",c[i][j]);
    }
    printf("\n");
  }
	return 0;
}

