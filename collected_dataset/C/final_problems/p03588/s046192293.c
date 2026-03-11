#include<stdio.h>
#include <stdlib.h>
int main(){
 int i,min,val;
 static int n;
 int *A,*B;
 A = malloc(100000000);
 B = malloc(100000000);
 min=1;
 scanf("%d",&n);
 for(i=0;i<n;i++){
 scanf("%d %d",&A[i],&B[i]);
 if (min<A[i]) min =A[i],val=A[i]+B[i];
}
printf("%d",val);
free(A);
free(B);
return 0;
}