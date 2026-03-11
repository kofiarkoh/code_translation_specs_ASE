#include<stdio.h>

int culc(int a,int b,int c){
  int s,t;
  s = a-b;
  if(s < 0)
    s *= -1;
  t = b-c;
  if(t < 0)
    t *= -1;

  return s + t;
}

int main(){
  int i,j,a,b,c;
  int cost[6];
  int min = 10000;
  scanf("%d %d %d",&a,&b,&c);

  cost[0] = culc(a,b,c);
  cost[1] = culc(a,c,b);
  cost[2] = culc(b,a,c);
  cost[3] = culc(b,c,a);
  cost[4] = culc(c,b,a);
  cost[5] = culc(c,a,b);

  for(i=0;i<6;i++){
    if(cost[i] < min)
      min = cost[i];
  }
  printf("%d\n",min);

  return 0;
}