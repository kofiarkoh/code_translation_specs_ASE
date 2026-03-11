#include <stdio.h>
    
typedef struct pair* ppair;
    
typedef struct pair {
  int first, second;
} pair;
    
    
int size;
pair v[1000000];
int psize;
pair preorder[1000000];
int sorted;
    
void swap(ppair a, ppair b) {
  pair t = *a;
  *a = *b;
  *b = t;
}
    
int getp(int begin, int end) {
  int i;
  int p = begin;
  for(i = begin+1; i < end; ++i) {
    if(v[i].first < v[p].first) {
      p = i;
    }
  }
  swap(&v[end-1], &v[p]);
  return end-1;
}
    
void qsort(int begin, int end) {
  int i, j;
  if(begin >= end) return;
  getp(begin,end);
  preorder[psize++] = v[end-1];
  if(begin+1 == end) return;
  for(i = begin, j = begin; i < end; ++i) {
    if(v[i].second < v[end-1].second) {
      swap(&v[i], &v[j++]);
    }
  }
  swap(&v[j],&v[end-1]);
  qsort(begin, j);
  qsort(j+1, end);
}
    
int binary_search(int k) {
  int l, r;
  l = 0;
  r = size;
  while(l < r) {
    int m = (l+r)/2;
    if(v[m].second == k) {
      return m;
    } else if(v[m].second < k) {
      l = m+1;
    } else {
      r = m;
    }
  }
  return -1;
}
    
int main() {
  int i, j;
  int n, k;
  char s[100];
  size = 0;
  sorted = 0;
  scanf("%d", &n);
  for(i = 0; i < n; ++i) {
    scanf("%s", s);
    if(s[0] == 'i') {
      scanf("%d", &k);
      v[size] = (pair){size,k};
      ++size;
      sorted = 0;
    } else if(s[0] == 'p') {
      if(!sorted) {
        psize = 0;
        qsort(0, size);
        sorted = 1;
      }
      for(j = 0; j < size; ++j) {
        printf(" %d", v[j].second);
      }
      printf("\n");
      for(j = 0; j < size; ++j) {
        printf(" %d", preorder[j].second);
      }
      printf("\n");
    } else {
      scanf("%d", &k);
      if(!sorted) {
        psize = 0;
        qsort(0, size);
        sorted = 1;
      }
      printf(binary_search(k)!=-1?"yes\n":"no\n");
    }
  }
  return 0;
}