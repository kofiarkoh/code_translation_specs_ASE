#include<stdio.h>
int main(){
  unsigned long long x,y,count=0;
  scanf("%lld %lld",&x,&y);
  while(y>=x){
    y/=2;
    count++;
  }
  printf("%d\n",count);
}