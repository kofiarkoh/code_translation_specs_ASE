#include <stdio.h>

int main(){
  long long int a,b;
  scanf("%lld",&a);
  if((0<a%11) && (a%11<7)){
    b= (long long int)(a/11) * 2 + 1;}
  else if(a%11==0){
    b = (long long int)(a/11) * 2;}
  else{
    b = (long long int)(a/11) + 2;}
  printf("%lld",b);
  return 0;
  }
