#include<stdio.h>
int main(){
  int n,i=1;
  scanf("%d",&n);
  while(n-i>0&&n-i>i+1){
    printf("%d\n",i);
    n-=i;
    i++;
  }
  printf("%d",n);
}