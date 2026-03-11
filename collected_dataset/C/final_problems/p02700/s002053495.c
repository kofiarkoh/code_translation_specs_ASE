#include<stdio.h>

int main(void){
  int a,b,c,d;
  scanf("%d %d %d %d",&a,&b,&c,&d);
  while(1){
    if( (c=c-b) <= 0 ){
       printf("Yes");
      return 0;
    }
    else if( (a=a-d) <=0){
      printf("No");
      return 0;
    }
  }
}
