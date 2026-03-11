#include <stdio.h>

int main(){
  int i, j;
  int n, m, d;
  int a[10];
  int r[10];

  scanf("%d%d%d", &n, &m, &d);

  for(i = 0; i < m; i++)
    scanf("%d", a + i);

  for(i = 0; i < d; i++){
    for(j = 0; j < m; j++)
      scanf("%d", r + j);

    while(n >= 0){
      int ok = 1;

      for(j = 0; j < m; j++){
	if(r[j] != -1 && n % a[j] != r[j]){
	  ok = 0;
	  break;
	}
      }

      if(ok) break;
      n--;
    }
  }

  printf("%d\n", n);

  return 0;
}