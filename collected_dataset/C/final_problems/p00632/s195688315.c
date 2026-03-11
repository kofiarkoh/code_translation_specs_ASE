#include <stdio.h>
#include <string.h>

int map[22][22];

void walk(int x, int y, int step)
{
    map[y][x] = step;
    
    if (map[y][x - 1] == -1 || map[y][x - 1] > step + 1){
        walk(x - 1, y, step + 1);
    }
    
    if (map[y][x + 1] == -1 || map[y][x + 1] > step + 1){
        walk(x + 1, y, step + 1);
    }

    if (map[y - 1][x] == -1 || map[y - 1][x] > step + 1){
        walk(x, y - 1, step + 1);
    }

    if (map[y + 1][x] == -1 || map[y + 1][x] > step + 1){
        walk(x, y + 1, step + 1);
    }
}


int main(void)
{
    int H, W;
    char pattern[11];
    char str[21];
    int ax, ay, bx, by;
    int x, y, minx, miny;
    int min;
    int len;
    int i;
    
    while (1){
        scanf("%d%d", &H, &W);
        if (H == 0 && W == 0){
            break;
        }
        
        for (y = 0; y <= H + 1; y++){
            for (x = 0; x <= W + 1; x++){
                map[y][x] = -2;
            }
        }

        for (y = 1; y <= H; y++){
            scanf("%s", str);
            for (x = 1; x <= W; x++){
                switch (str[x - 1]){
                  case '#':
                    map[y][x] = -2;
                    break;

                  case '.':
                    map[y][x] = -1;
                    break;

                  case 'A':
                    map[y][x] = 0;
                    ax = x;
                    ay = y;
                    break;

                  case 'B':
                    map[y][x] = -1;
                    bx = x;
                    by = y;
                    break;
                }
            }
        }
        scanf("%s", pattern);
        
        walk(ax, ay, 0);
#if 0
        for (y = 0; y <= H + 1; y++){
            for (x = 0; x <= W + 1; x++){
                printf("%2d ", map[y][x]);
            }
            printf("\n");
        }
#endif
        
        min = 1000;
        i = 0;
        len = strlen(pattern);
        while (1){
            switch (pattern[i % len]){
              case '2':
                if (by < H){
                    by++;
                }
                break;
        
              case '4':
                if (bx > 1){
                    bx--;
                }
                break;
        
              case '6':
                if (bx < W){
                    bx++;
                }
                break;

              case '8':
                if (by > 1){
                    by--;
                }
                break;
            }
// printf("[%c %d=%d %d, %d]", pattern[i%len], i+ 1,bx, by, map[by][bx]);
            if (map[by][bx] >= 0 && map[by][bx] <= i + 1){
                if (min > i + 1){
                    min = i + 1;
                    minx = bx;
                    miny = by;
                }
            }
            i++;
            if (i > 400){
                break;
            }
        }
#if 0        
        if (map[by][bx] > 0){
            if (min > map[by][bx]){
                min = map[by][bx];
                minx = bx;
                miny = by;
            }
        }
#endif
        if (min == 1000){
            printf("impossible\n");
        }
        else {
            printf("%d %d %d\n", min, miny - 1, minx - 1);
        }
    }
    return (0);
}

