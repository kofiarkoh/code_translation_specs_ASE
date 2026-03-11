#include<stdio.h>
#define N 1000

int main(void){
  int i,num,j,k;
  int n[N];
  int ms[15][15];
  int curX,curY;

  for(i=0;;i++){
    scanf("%d",&n[i]);
    if(n[i]==0)
      break;
  }
  num=i;
  for(i=0;i<num;i++){
    for(j=0;j<n[i];j++){
      for(k=0;k<n[i];k++){
        ms[j][k]=0;
      }
    }
    curX=n[i]/2+1;
    curY=n[i]/2;
    ms[curX][curY]=1;
    for(j=2;j<=n[i]*n[i];j++){
      curX++;
      curY++;
      if(curX>=n[i])
        curX=0;
      if(curY>=n[i])
        curY=0;
      if(ms[curX][curY]!=0){
        curX++;
        curY--;
        if(curX>=n[i])
          curX=0;
        else if(curX==-1)
          curX=n[i]-1;
        if(curY>=n[i])
          curY=0;
        else if(curY==-1)
          curY=n[i]-1;
      }
      ms[curX][curY]=j;
    }
    for(j=0;j<n[i];j++){
      for(k=0;k<n[i];k++){
        printf("%4d",ms[j][k]);
      }
      printf("\n");
    }
  }
  return 0;
}