#include <stdio.h>

int main()
{
    int x,y,z;
    scanf("%d %d", &x, &y);
    if(x-y < 0)
      z=-(x-y);
    else z=x-y;
    
    printf("%d\n", z+(y+1));

    return 0;
}

