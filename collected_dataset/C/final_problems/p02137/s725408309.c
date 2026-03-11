#include<stdio.h>

int main(void){
  int P,x,y=0;
  scanf("%d",&P);
  x=P/500;
  x=x*500;
  if(x==0){
    printf("0\n");
  }
  else{
    printf("%d\n",x);
  }
}

