#include<stdio.h>

int main() {
  int n, cnt = 0, max, min, maxcnt = 0, mincnt = 0;
  scanf("%d", &n);
  int a[n];
  for(int i = 0; i < n; i++) {
    scanf("%d", &a[i]);
  }
  min = a[0];
  max = a[0];
  for(int i = 0; i < n; i++) {
    if(a[i] < min) {
      min = a[i];
    }
    if(a[i] > max) {
      max = a[i];
    }
  }
  
  for(int i = 0; i < n; i++) {
    if(a[i] == max) {
      maxcnt++;
    }
    if(a[i] == min) {
      mincnt++;
    }
  }
  cnt = max - min;
  if(cnt == 0) {
    printf("%d\n", max);
  } else if(cnt == 1) {
    if(maxcnt > mincnt) {
      printf("%d", max);
    } else if(maxcnt < mincnt) {
      printf("%d", min);
    } else {
      printf("%d", cnt);
    } 
  }else{
    printf("%d\n", cnt);
  }
  return 0;
}