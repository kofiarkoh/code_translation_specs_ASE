#include<stdio.h>
#define MAX 100000

int main(){
 int N,T;
 int A[MAX]={0};
 int i,l,r,j;
 int max=-1;

 scanf("%d %d",&N,&T);

 for(i=0;i<N;i++){
  scanf("%d %d",&l,&r);
  for(j=l;j<r;j++)A[j]++;
 }

 for(i=0;i<=T;i++)
  if(A[i]>max)max=A[i];

 printf("%d\n",max);

return 0;
}
