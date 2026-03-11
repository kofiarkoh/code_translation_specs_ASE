#include <stdio.h>


int main(void)
{
    int a, b;
    char c;
    int ctr[5];
    
    
    ctr[1] = ctr[2] = ctr[3] = ctr[4] = 0;
    
    
    scanf("%d%c%d", &a, &c, &b);
    ctr[a]++;
    ctr[b]++;
    scanf("%d%c%d", &a, &c, &b);
    ctr[a]++;
    ctr[b]++;
    scanf("%d%c%d", &a, &c, &b);
    ctr[a]++;
    ctr[b]++;
    
    
    if(ctr[1] < 3 && ctr[2] < 3 && ctr[3] < 3 && ctr[4] < 3)
    {
        printf("YES\n");
    }
    else
    {
        printf("NO\n");
    }
    
    //system("pause");
    
    return 0;
}
