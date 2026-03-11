#include<stdio.h>


int main(void){

  int d;
  int space;
  int i;

  while(scanf("%d", &d)!=EOF){
    i=1;
    space=0;
    while(d*i!=600){
      space+=d*(d*i)*(d*i);
      i++;
    } 
    printf("%d\n", space);
  }

  return(0);
}

