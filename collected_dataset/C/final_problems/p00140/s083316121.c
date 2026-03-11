#include<stdio.h>

int main(void){
  int st[21]={0,1,2,3,4,5,6,7,8,9,5,4,3,2,1,0};
  int n,start,end,i,j;

  scanf("%d",&n);

  for(i=0;i<n;i++){
    scanf("%d%d",&start,&end);

    if(start>end){
      start=15-start;
      end=15-end;
    }
    for(j=start;j<=end;j++){
      printf("%d",st[j]);
      printf((j!=end)?" ":"\n");
    }
  }
  return 0;
}