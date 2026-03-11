#include <stdio.h>
#include <math.h>

int main()
{
    int t, x1, x2, x3, x4, y1, y2, y3, y4;
    double  m1, m2, temp;
    scanf("%d", &t);
    while(t--)
    {
        scanf("%d %d %d %d %d %d %d %d", &x1, &y1, &x2, &y2, &x3, &y3, &x4, &y4);

        if(((x2-x1)*(x4-x3))+((y2-y1)*(y4-y3))==0)
        {
            printf("1\n");
        }
        else if(((x2-x1)*(y4-y3))==((y2-y1)*(x4-x3)))
        {
            printf("2\n");
        }
        else
        {
            printf("0\n");
        }
    }
}

