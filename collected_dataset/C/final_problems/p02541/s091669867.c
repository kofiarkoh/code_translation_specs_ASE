#include<stdio.h>

int main(void){
  long long n;
  scanf("%lld",&n);
  long long i=1;
  if(n<=100){
  long long i=1;
  }else if(n<=1000){
  long long i=13;
  }else if(n<=5000){
  long long i=100;
  }else if(n<=10000){
  long long i=140;
  }else if(n<=100000){
  long long i=445;
  }else{
    long long i=1100140;
  }
  
  while((i*(i+1)/2)%n!=0){
    i++;
  }
  printf("%lld",i);
  return 0;
}