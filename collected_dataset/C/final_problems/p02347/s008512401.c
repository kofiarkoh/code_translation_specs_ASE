#include<stdio.h>
#define NMAX 50000
#define QMAX 20000


int main(){
int n,i,q,j;
int X[NMAX];
int Y[NMAX];
int SX[QMAX];
int TX[QMAX];
int SY[QMAX];
int TY[QMAX];

scanf("%d",&n);
for(i=0;i<n;i++)scanf("%d %d",&X[i],&Y[i]);

scanf("%d",&q);
for(i=0;i<q;i++)scanf("%d %d %d %d",&SX[i],&TX[i],&SY[i],&TY[i]);

for(i=0;i<q;i++){
 for(j=0;j<n;j++){
  if(X[j]>=SX[i]&&X[j]<=TX[i]&&Y[j]>=SY[i]&&Y[j]<=TY[i])printf("%d\n",j);
 }
 printf("\n");
}

return 0;
}
