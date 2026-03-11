#include <stdio.h>
int main(void){
    // Your code here!
    
  int a,b,c;
  scanf("%d %d %d",&a,&b,&c); 
  int min=a,max=a,x,y;
  if(min>b) {
      x=min; min=b; b=x;
  }
  if(min>c){
      x=min; min=c; c=x;
  }
 if(max<b){
     y=max; max=b; b=x;
 }
 if(max<c){
     y=max; max=c; c=x;
 }
 printf("%d %d\n",min,max);
    return 0;
  }

