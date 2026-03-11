#include<stdio.h>
#define MAX 100000

int n, A[MAX+1];

void swap(int *a, int *b){
  int temp;
  temp = *a;
  *a = *b;
  *b = temp;
}
void maxHeapify(int i){
  int l, r, largest;
  l = 2 * i;
  r = 2 * i + 1;

  if(l <= n && A[l] > A[i]) largest = l;
  else largest = i;
  if(r <= n &&A[r] > A[largest]) largest = r;

  if(largest != i){
    swap(&A[i], &A[largest]);
    maxHeapify(largest);
  }
}

int main(){
  int i;
  
  scanf("%d",&n);

  for(i = 1; i <= n; i++) scanf("%d",&A[i]);

  for(i = n / 2; i >= 1; i--) maxHeapify(i);

  for(i = 1; i <= n; i++) printf(" %d",A[i]);
  printf("\n");

  return 0;
}