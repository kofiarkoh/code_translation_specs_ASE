#include<stdio.h>

void solve(void);

int main(void){
  int T;
  scanf("%d",&T);
  while(T--) solve();
  return 0;
}

void solve(void){
  int N,D,x,y,sum=0,i;
  scanf("%d%d",&N,&D);
  x=D;
  for(i=0;i<N-1;i++){
    y=x^127;
    sum+=x^y;
    x=y;
  }
  printf("%d\n",sum+x);
}
