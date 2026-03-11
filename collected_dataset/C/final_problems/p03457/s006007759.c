#include<stdio.h>

int main(){

  int N,t1=0,t2,x1=0,x2,y1=0,y2,ans=0;
  scanf("%d",&N);
  for (int i = 0; i < N; i++)
  {
    scanf("%d%d%d",&t2,&x2,&y2);
    if (t2-t1>=x2-x1+y2-y1)
    {
      if(((t2-t1)-(x1-x2+y1-y2))%2==0){
        t1=t2; x1=x2; y1=y2;
        continue;
      }else{
        ans=1;
        break;
      } 
    }else{
      ans=1;
      break;
    }
  }

  ans > 0 ? puts("No"):puts("Yes");

  return 0;
}