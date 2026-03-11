#include<stdio.h>

int main()
{
    int n;
    int k;
    int h = 0;
    int w = 0;
    int s = 1;
    int b = 0;
    
    scanf("%d%d", &n, &k);
    
    while(1)
    {
        if(w <= k * s)
        {
            w += s;
            b += s;
            if(b <= n)
            {
                h++;
            }
            else
            {
                break;
            }
        }
        else
        {
            s++;
        }
    }
    
    printf("%d\n", h);
    
    return 0;
}
