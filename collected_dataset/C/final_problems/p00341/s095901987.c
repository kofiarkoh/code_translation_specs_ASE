#include<stdio.h>

void sort(int *a, int N) {
  int i, tmp, flag, k;
  k = 0;
  do {
    flag = 0;
    for (i = 0; i < N-1-k;i++) {
      if(a[i] > a[i+1]) {
	flag = 1;
	tmp = a[i];
	a[i] = a[i+1];
	a[i+1] = tmp;
      }
    }
    k++;
  } while(flag == 1);
}

int main(void) {
  int a[12], i;
  for (i = 0; i < 12; i++) {
    scanf("%d", &a[i]);
  }
  sort(a, 12);

  if(a[0]==a[3]&&a[4]==a[7]&&a[8]==a[11]) printf("yes\n");
  else printf("no\n");

  return 0;
}