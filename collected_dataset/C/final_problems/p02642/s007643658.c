#include <stdio.h>
#include <stdlib.h>

int compare_ints(const void* a, const void* b)
{
    int arg1 = *(const int*)a;
    int arg2 = *(const int*)b;
 
    if (arg1 < arg2) return -1;
    if (arg1 > arg2) return 1;
    return 0;
}

int	main(void)
{
  	int	n;
  	int	a[200002];
  	int	i;
  	int	j;
  	int	cnt;
  	int flg;
  
  	scanf("%d\n", &n);
  	for(i=0; i<n; i++)
    {
      	scanf("%d", &a[i]);
    }
  	
  	qsort(a, n, sizeof(int), compare_ints);

    cnt = n;
    flg = 0;
  	for(i=1; i<n; i++)
    {
        if (a[0] == a[i])
        {
            flg = 1;
        }
/*
        else
        {
            flg = 0;
            break;
        }
*/
    }
    if(flg == 1)
    {
        cnt--;
    }
    
  	for(i=1; i<n; i++)
    {
      	for(j=0; j<i; j++)
        {
          	if (a[i] % a[j] == 0)
            {
 //               printf("%d / %d\n", a[i], a[j]);
                cnt--;
                break;
            }
        }
    }
  	printf("%d", cnt);
  	return (0);
}