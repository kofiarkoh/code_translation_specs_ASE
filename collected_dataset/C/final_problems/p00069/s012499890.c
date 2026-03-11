#include <stdio.h>
#define WIDTH_MAX 10
#define HEIGHT_MAX 30

char amida[HEIGHT_MAX][WIDTH_MAX + 1];
int height, width, goal, start;

int check()
{
    int x, y;

    for(y = 0, x = start; y < height; y++)
    {
        if(amida[y][x] == '1')
            x++;
        else if(x > 0 && amida[y][x - 1] == '1')
            x--;
    }

    return x == goal;
}

int main(void)
{
    int x, y;

  next:

    while(scanf("%d", &width), width)
    {
        scanf("%d %d %d%*c", &start, &goal, &height);
        start--, goal--;       /* start and goal are given 1-origin */

        for(y = 0; y < height; y++)
            gets(amida[y]);

        if(check())
        {
            puts("0");
            goto next;
        }

        for(y = 0; y < height; y++)
            for(x = 0; x < width; x++)
                if(amida[y][x] != '1')
                {
                    amida[y][x] = '1';
                    if(check())
                    {
                        printf("%d %d\n", y + 1, x + 1);
                        goto next;
                    }
                    amida[y][x] = '0';
                }

        puts("1");
    }

    return 0;
}