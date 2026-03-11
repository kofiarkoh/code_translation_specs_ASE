#include <stdio.h>

int main(){
  int n,a[2][100];
  int ame,ame_max;
  int i,j,down;
  scanf("%d",&n);
  for(i = 0;i < 2;i++){
    for(j = 0;j < n;j++){
      scanf("%d",&a[i][j]);
    }
  }

  ame_max = 0;
  for(i = 0;i < n;i++){
    down = 0;
    ame = 0;
    for(j = 0;j < n;j++){
      ame += a[down][j];
      if(j == i){
        down++;
        ame+= a[down][i];
      }
    }
    if(ame > ame_max){
      ame_max = ame;
    }
  }

  printf("%d\n",ame_max);
  return(0);
}