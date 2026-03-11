#include<stdio.h>
 

int gcd(int x,int y){
  return y == 0 ? x : gcd(y, x%y);
}


int main(){

  int a,b;

  while(scanf("%d%d",&a,&b)!= EOF){
    printf("%d\n",gcd(a,b));
  }
  return 0;
}