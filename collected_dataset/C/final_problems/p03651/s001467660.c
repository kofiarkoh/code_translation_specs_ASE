#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <ctype.h>
#include <string.h>
#define ll long long
#define MAX 20000000000
#define MOD 1000000007

int main(void){
ll int N,K;
int A[100001]={0};
 scanf("%lld%lld",&N,&K);
int i,flag=1;
ll int min=MAX,min2=MAX;
for(i=0;i<N;i++){
  scanf("%d",&A[i]);
  if(K==A[i]) { printf("POSSIBLE\n"); exit(0);}
  if(A[i]>K) flag=0;
 
  if(min>A[i]) {min2=min; min=A[i];}
  else if(min2>A[i]) min2=A[i];
}
if(flag==1){ printf("IMPOSSIBLE\n"); exit(0);}
int lc;
for(i=2;i<=min;i++) if(min%i==0) {lc=i; break;}

flag=1;
int ppp=0;

for(i=0;i<N;i++){
 if(A[i]%lc) { flag=0; break;}
 if(i==N-1) ppp=1;
}
if(min==1) flag=0;
if(K%lc==0) flag=0;

if(ppp==1){
for(i=0;i<N;i++){
 if(min!=A[i]) break;
 if(i==N-1) flag=1;
}
for(i=0;i<N;i++){
  if(A[i]%min) break;
  if(i==N-1&&min!=lc) flag=1;}

  if(K<min2-min) flag=1;
  else flag=0;
  if(K%lc) flag=1;
}


if(flag==1) printf("IMPOSSIBLE\n");
else printf("POSSIBLE\n");

 return 0;
}