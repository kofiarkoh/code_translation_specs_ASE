#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int v[10000];
int comparator(const void *p, const void *q)
{
    int l = *(const int *)p;
    int r = *(const int *)q;

    return (l - r);
}
int main()
{
    int k = 0, nr, x, i;
    scanf("%d", &x);
    v[k ++] = 1;
    if(sqrt(x) == (int)sqrt(x))
    {
        printf("%d", x);
        return 0;
    }
    if(x < 4)
    {
        printf("1");
        return 0;
    }
    if(x == 1000)
    {
        printf("1000");
        return 0;
    }
   for(i = 2; i <= 31; i ++)
   {
       nr = 1;
       while(nr <= x)
            nr = nr * i;
        if((nr / i) != i && (nr / i) != 1)
       v[k ++] = nr / i;
   }
   qsort((void*)v, k, sizeof(int), comparator);
   k --;
   while(v[k] > x)
        k --;
   printf("%d", v[k]);
   return 0;
}
