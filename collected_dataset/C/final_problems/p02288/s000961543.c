

#include<stdio.h>
#define MAX 1000000
int A[MAX+1],n;
void maximumHeap(int);


void maximumHeap(int i){
  int l,r,largest;
  l=2*i;
  r=2*i+1;

  if(l<=n && A[l]>A[i]){
    largest=l;
 
  }
  else{
    largest=i;
  
  }
  if(r<=n && A[r]>A[largest]){
    largest=r;
  
  }
  
  if(largest != i){
 
  int k;
  k=A[i];
  A[i]=A[largest];
  A[largest]=k;
  maximumHeap(largest);
  }
}


int main(){
  
  int i;
  
  scanf("%d",&n);
  
  for(i=1;i<=n;i++){
    scanf("%d",&A[i]);
  }
  
  for(i=n/2;i>=1;i--){
    maximumHeap(i);
  }

  for(i=1;i<=n;i++){
    printf(" %d",A[i]);
  }
  printf("\n");
  return 0;
}