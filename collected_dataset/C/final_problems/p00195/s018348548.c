#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>
#include <time.h>
#include <ctype.h>

int cmp(const void *a, const void *b) { return *(int*)a - *(int*)b; }
int max(int a, int b) { return a >= b ? a : b; }
int min(int a, int b) { return a <= b ? a : b; }
int dsum(int x) { int r=0; while(x){ r+=(x%10); x/=10; } return r;}
void swap(int *a, int *b) { int tmp; tmp = *a; *a = *b; *b = tmp; return; }

int main(void) {
  int a,b,idx,cnt=0,ans=0;
  char names[6] = {'A','B','C','D','E'};
  while(scanf("%d%d",&a,&b) != EOF) {
    if(cnt == 5) {
      printf("%c %d\n",names[idx],ans);
      cnt = 0;
      ans = 0;
    }
    if(a+b > ans) {
      ans = a+b;
      idx = cnt;
    }
    cnt++;
  }
  return 0;
}
