#include <stdio.h>
int main()
{
  int i, j, n, A[10000], count = 0;

  scanf("%d",&n);
  for (i = 0; i < n; i++){
    scanf("%d",&A[i]);
  }
  for (i = 0; i < n; i++){
    if (A[i] > 1){
      for (j = 2; j*j <= A[i]; j++)
	if (A[i] % j == 0){
	  count++;
	  break;
	}
    }
    else
      count++;
  }
  printf("%d\n",n-count);

  return 0;
}

