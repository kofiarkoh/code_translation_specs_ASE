#include<stdio.h>
#include<stdlib.h>
int D(int d,int dd);
typedef struct pp{
  int id;
  double k;
  int parent;
  int depth;
  int C[100];
  int J;
}P;
P *A;
int main(){
  int n,*S,i,j,N,count=0;
  
  scanf("%d\n",&n);
  
  A=(P*)malloc((n*n)*sizeof(P));
  for(i=0;i<n*n;i++) A[i].k=3.14;
  for(i=1;i<=n;i++) scanf("%lf",&A[i].k);
  
  printf("node 1: key = %.0f, left key = %.0f, right key = %.0f,\n",A[1].k,A[2].k,A[3].k);
  for(i=2;i<=n;i++){
    if(A[i*2].k==3.14) printf("node %d: key = %.0f, parent key = %.0f, \n",i,A[i].k,A[i/2].k);
    else if(A[i*2].k!=3.14&&A[i*2+1].k==3.14) printf("node %d: key = %.0f, parent key = %.0f, left key = %.0f, \n",i,A[i].k,A[i/2].k,A[i*2].k);
    else printf("node %d: key = %.0f, parent key = %.0f, left key = %.0f, right key = %.0f, \n",i,A[i].k,A[i/2].k,A[i*2].k,A[i*2+1].k);
  }
 
  return 0;
}



