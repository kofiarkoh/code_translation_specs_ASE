#include<stdio.h>

int main()
{
    int w;
    int h;
    int a[1000] = {};
    int b[1000] = {};
    int sum_a = 0;
    int sum_b = 0;
    int tmp;
    int i;
    int j;
    int k;
    
    scanf("%d%d", &w, &h);

    for(i = 0; i < w; i++)
    {
        scanf("%d", &a[i]);
        sum_a += a[i];
    }
    
    for(i = 0; i < h; i++)
    {
        scanf("%d", &b[i]);
        sum_b += b[i];
    }
    
    if(sum_a != sum_b)
    {
        puts("0");
        return 0;
    }

    for(i = 0; i < w; i++)
    {
        for(j = 0; j < h; j++)
        {
            for(k = j + 1; k < h; k++)
            {
                if(b[j] < b[k])
                {
                    tmp = b[j];
                    b[j] = b[k];
                    b[k] = tmp;
                }
            }
        }
        
        for(j = 0; j < h; j++)
        {
            if(!a[i] || !b[j])
            {
                break;
            }
            a[i]--;
            b[j]--;
        }
        
        if(a[i] > 0)
        {
            puts("0");
            return 0;
        }
    }
    
    puts("1");

    return 0;
}
