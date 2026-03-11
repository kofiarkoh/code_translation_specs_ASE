#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

void swap(int *A, int i, int j) {
  int temp;
  temp = A[i];
  A[i] = A[j];
  A[j] = temp;
}

void Merge(int *A, int left, int mid, int right) {
  int n1,n2,*L,*R,i,j,k;
  n1 = mid -left;
  n2 = right - mid;
  L = (int*)malloc(sizeof(int) * (n1+1));
  R = (int*)malloc(sizeof(int) * (n2+1));
  for (i = 0; i< n1; i++) {
    L[i] = A[left+i];
  }
  for (i = 0; i < n2; i++) {
    R[i] = A[mid+i];
  }
  L[n1] = INT_MAX; R[n2] = INT_MAX;
  i = 0;j = 0;
  for(k = left; k < right; k++) {
    if (L[i] < R[j]) {
      A[k] = L[i];
      i++;
    }else {
      A[k] = R[j];
      j++;
    }
  }
  free(L);
  free(R);
}

void Merge_Sort(int *A, int left, int right) {
  if(left+1 < right) {
    int mid = (left + right)/2;
    Merge_Sort(A,left,mid);
    Merge_Sort(A,mid,right);
    Merge(A,left,mid,right);
  }
}

int sort_step(int *w, int *w1, int n) {
  int i,j,mini,minj,min;
  min = INT_MAX;
  for (i = 0; i < n; i++) {
    j = 0;
    while(w1[i] != w[j]) j++;
    if(i != j) {
      if(min > w[i]+w[j]) {
	mini = i;
	minj = j;
	min = w[i]+w[j];
      }
    }
  }
  if(min == INT_MAX) return 0;
  swap(w,mini,minj);
  return min;
}

long min_sort(int *w, int *w1, int n) {
  long sum = 0,tmp = 0;
  while(1) {
    sum += sort_step(w,w1,n);
    if(sum == tmp) break;
    tmp = sum;
  }
  return sum;
}

int main()
{
  int i,n,*w,*w2;
  long sum;
  scanf("%d",&n);
  w = (int*)malloc(sizeof(int) * n);
  w2 = (int*)malloc(sizeof(int) * n);
  for (i = 0; i < n; i++) {
    scanf("%d",w+i);
    w2[i] = w[i];
  }
  Merge_Sort(w2,0,n);
  
  sum = min_sort(w,w2,n);
  printf("%ld\n",sum);
  
  return 0;
}