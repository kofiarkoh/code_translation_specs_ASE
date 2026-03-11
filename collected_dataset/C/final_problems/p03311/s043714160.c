#include <stdio.h>
#include <stdlib.h>

int n;
int A[200000];

inline void swap(int *a, int *b){
  int z;
  z = *a;
  *a = *b;
  *b = z;
}

int kth(int *A, int n, int k){
  int ll, rr;
  ll = 0;
  rr = n;
  while(1){
    int i, l, r, pivot;
/*
    if(A[n/2] > A[0])    swap(A+n/2, A);
    if(A[n/2] > A[n-1])  swap(A+n/2, A+n-1);
    if(A[0] > A[n-1])    swap(A, A+n-1);
*/
    swap(A+ll, A+(ll+rr)/2);
    pivot = A[ll];
    l = ll;
    r = rr;
    i = ll+1;
    while(i < r){
      if(A[i] < pivot) swap(A+l++, A+i++);
      else if(A[i] > pivot) swap(A+--r, A+i);
      else i++;
    } 
    if(ll+k < l) rr = l;
    else if(ll+k >= r){
      ll = r;
      k -= r;
    }
    else return pivot;
  }
}

int main(){
  int i, m;
  long long int r;
  scanf("%d", &n);
  for(i=0;i<n;i++){
    scanf("%d", A+i);
    A[i] -= i+1;
  }
  m = kth(A, n, n/2);
  r = 0;
  for(i=0;i<n;i++) r += abs(A[i] - m);
  printf("%lld\n", r);
  return 0;
}
