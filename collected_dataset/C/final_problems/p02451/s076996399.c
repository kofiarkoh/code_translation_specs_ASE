#include <stdio.h>
#include <stdbool.h>

#define MAX 100000

bool binary_search(int);

int A[MAX];
int n;

int main(void) {
  int q, k, i;

  scanf("%d", &n);
  for(i=0; i<n; ++i)
    scanf("%d", &A[i]);

  scanf("%d", &q);
  for(i=0; i<q; ++i) {
    scanf("%d", &k);
    if(binary_search(k)) {
      printf("1\n");
    }else {
      printf("0\n");
    }
  }

  return 0;
}

bool binary_search(int key) {
  int mid, left = 0, right = n;

  while(left < right) {
    mid = (left+right)/2;
    if(A[mid] == key) {
      return true;
    } else if(A[mid] < key) {
      left = mid+1;
    } else {
      right = mid;
    }
  }

  return false;
}

