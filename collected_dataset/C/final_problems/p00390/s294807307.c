#include<stdio.h>
#include<stdlib.h>

int main()
{
    int n;
    int *a;
    int w;
    int r = 0;
    int min_wr = 1001;
    int min_wl = 1001;
    int i;
    
    scanf("%d", &n);
    
    a = (int *)malloc(sizeof(int) * n);
    
    for(i = 0; i < n; i++)
    {
        scanf("%d", &a[i]);
        if(a[i] == 0)
        {
            r++;
        }
    }
    
    if((r == 0) || (r == n))
    {
        puts("0");
    }
    else
    {
        for(i = 0; i < n; i++)
        {
            scanf("%d", &w);
            if(a[i] == 0)
            {
                if(w < min_wr)
                {
                    min_wr = w;
                }
            }
            else
            {
                if(w < min_wl)
                {
                    min_wl = w;
                }                
            }
        }
        printf("%d\n", min_wr + min_wl);
    }
    
    return 0;
}
