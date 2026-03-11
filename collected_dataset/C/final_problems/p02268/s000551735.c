#include<stdio.h>
#define N 100000
int A[N],n;
int bsearch(int);
int main(){
  int key,a,i,j,sum=0;
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&A[i]);
  }
  scanf("%d",&a);
  for(j=0;j<a;j++){
    scanf("%d",&key);
    if(bsearch(key)){
      sum++;
    }
  }
  printf("%d\n",sum);
  return 0;
}
int bsearch(int k){
  int l=0,r,m;
  r=n;
  while(l<r){
    m=(r+l)/2;
    if(k==A[m]) return 1;
    if(k > A[m]){
      l = m+1;
    }
    else if(k < A[m]){
      r = m;
    }
  }
  return 0;
}