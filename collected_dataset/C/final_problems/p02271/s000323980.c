#include<stdio.h>

#define N 20
#define Q 200

void solve(int,int);

int A[N],M[Q];
int count,n;

int main(){ 
  int q,i;
  
  scanf("%d", &n);
  for( i = 0; i < n; i++ ) scanf("%d", &A[i]);
  scanf("%d", &q);
  for( i = 0; i < q; i++ ) scanf("%d", &M[i]);
  
  for(i=0;i<q;i++){
    count=0;
    solve(0,M[i]);
    if(count==0) printf("no\n");
  }
  return 0;
}

void solve(int i,int m){
  if(i<n && count==0 && m>0){
    if(A[i]==m){
      printf("yes\n");
      count++;
      return;
    }
  solve(i+1,m-A[i]);
  solve(i+1,m);
  }
}