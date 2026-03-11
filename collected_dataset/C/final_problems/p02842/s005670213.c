#include <stdio.h>

int main(void){
 int N,X1,X2;
 scanf("%d",&N);
  X1=(int)N/1.08;
  X2=X1+1;
 if((int)(X1*1.08)==N){printf("%d\n",X1);}
 else if((int)(X2*1.08)==N){printf("%d\n",X2);}
 else{printf(":(\n");}
}