#include<stdio.h>

#define ABS(a) ((a)>(0)?(a):-(a))

int gcd(int a,int b){
  int r=a%b;
  while(r>0){
    a=b;b=r;r=a%b;
  }
  return b;
}

void run(void){
  int a,b,c,d;
  scanf("%d%d%d%d",&a,&b,&c,&d);
  int dx=ABS(a-c);
  int dy=ABS(b-d);
  if(dx==0 || dy==0){
    puts("0");
    return;
  }
  int g=gcd(dx,dy);
  printf("%d\n",(dx/g+dy/g-1)*g);
}

int main(void){
  run();
  return 0;
}