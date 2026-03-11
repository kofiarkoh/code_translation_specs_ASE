#include<stdio.h>

int main(void){
  int h1,h2,k1,k2;
  int a,b,c,d;
  int hp=0,kp=0;
  scanf("%d %d",&h1,&h2);
  scanf("%d %d",&k1,&k2);
  scanf("%d %d %d %d",&a,&b,&c,&d);

  hp=a*h1+b*h2+c*h1/10+d*h2/20;
  kp=a*k1+b*k2+c*k1/10+d*k2/20;

  if(hp>kp) printf("hiroshi\n");
  else if(hp<kp) printf("kenjiro\n");
  else printf("even\n");

  return 0;
}

