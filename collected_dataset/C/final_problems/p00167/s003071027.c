#include <stdio.h>
int main(void) {
  int n, data[100], j, tmp, i, count;
  while(scanf("%d",&n),n) {
    count = 0;
    for(i = 0; i < n; i++ ) {
      scanf("%d",&data[i]);
    }
    for(i = 0; i < n - 1; i++ ) {
      for(j = 1; j < n - i; j++ ) {
	if(data[j] < data[j-1]) {
	  tmp = data[j];
	  data[j] = data[j-1];
	  data[j-1] = tmp;
	  count++;
	}
      }
    }
    printf("%d\n",count);
  }
  return 0;
}