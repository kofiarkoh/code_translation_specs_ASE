#include <stdio.h>

int main(){
  int i, j;
  int n, m, d;
  int a[10];
  int p[10];
  int r[10];

  scanf("%d%d%d", &n, &m, &d);

  for(i = 0; i < m; i++)
    scanf("%d", a + i);

  for(i = 0; i < m; i++)
    p[i] = a[i] * 100;


  for(i = 0; i < d; i++){
    for(j = 0; j < m; j++)
      scanf("%d", r + j);

    for(j = 0; j < m; j++)
      if(r[j] != -1)
	r[j] = (n % a[j] + a[j] - r[j]) % a[j];

    while(n >= 0){
      int ok = 1;
      int mx = 0;

      for(j = 0; j < m; j++)
	if(r[j] > 0){ ok = 0; break; }

      if(ok) break;

      for(j = 0; j < m; j++)
	if(r[j] > mx) mx = r[j];

      for(j = 0; j < m; j++)
	if(r[j] >= 0){
	  r[j] = (r[j] + p[j] - mx) % a[j];
	}

      n -= mx;
    }
  }

  if(n < 0) puts("-1");
  else printf("%d\n", n);

  return 0;
}