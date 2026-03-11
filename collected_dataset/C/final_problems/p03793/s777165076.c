#include <stdio.h>

int main()
{
  char s[500001];
  int Q;
  int l[100001];
  int r[100001];
  int i, j, k;
  int n;
  int q;
  char buf[500001];
  int flag;
  int ans;
  int owaowari;
  
  scanf("%s", &s);
  scanf("%d", &Q);
  for(i = 0; i < Q; i++) {
    scanf("%d %d", &l[i], &r[i]);
  }
  
  for(q = 0; q < Q; q++) {
    n = 0;
    for(i = l[q]-1; i < r[q]; i++) {
      buf[n] = s[i];
      n++;
    }
    buf[n] = '\0';
    ans = -1;
    while(n > 1) {
      //printf("%s\n", buf);
      flag = -1;
      for(i = 0; i < n; i++) {
	if(buf[i] == buf[i+1]) {
	  flag = 1;
	  if(buf[i] == 'z') {
	    for(j = i; j < n-2; j++) {
	      buf[j] = buf[j+2];
	    }
	    //	    printf("%d\n", j);
	    buf[j] = '\0';
	    n -= 2;
	    //	    printf("%s\n", buf);
	    if(n == 0) {
	      ans = 1;
	      break;
	    }
	  } else {
	    buf[i] += 1;
	    for(j = i+1; j < n; j++) {
	      buf[j] = buf[j+1];
	    }
	    buf[j] = '\0';
	    n -= 1;
	    //printf("%s\n", buf);
	  }
	} 
      }
      if(ans == 1) {
	break;
      }
      if(flag == -1) {
	break;
      }
    }
    if(ans == 1) {
      printf("Yes\n");
    } else {
      printf("No\n");
    }
  }
  
  return 0;
}
