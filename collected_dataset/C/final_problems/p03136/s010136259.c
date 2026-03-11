#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main(void){

  int i,N,max=0,a=0;
  scanf("%d",&N);
  int l[N];
  
   for(i=0;i<N;i++){
     scanf("%d",&l[i]);
     if(max<=l[i]){
       max=l[i];
     }
     a+=l[i];
   }
  if(a>2*max)
     printf("Yes\n");
  else
    printf("No");
}