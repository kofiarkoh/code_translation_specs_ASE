#include <stdio.h>
#include <stdlib.h>

int main(void)
{
  long int n;
  scanf("%ld", &n);
  
  long int* array = malloc(n * sizeof(long int));
  
  int i, j;
  for (i = 0; i < n; ++i)
  {
    scanf("%ld", &array[i]);
  }
  
  long int a = 0;
  for (i = 0; i < n; ++i)
  {
    a ^= array[i];
  }
  long int ans;
  for (i = 0; i < n; ++i)
  {
    ans = a ^ array[i];
    printf("%ld ", ans);
  }
  printf("\n");
  
  
  free(array);
  
  return 0;
}
