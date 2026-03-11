#include<stdio.h>
int main(){
  int x,i;
  scanf("%d",&x);
  for(i=0;i*(i+1)/2<x;i++);
  printf("%d",i);
}