#include <stdio.h>

int main(void)
{
    int H, W;
    int a[50][50];
    int b[50][50];
    int h, w;
    int c[50][50];
    int sum, max;
    int flag;
    int ax, ay, bx, by, cx, cy;
    
    scanf("%d%d", &H, &W);
    for (ay = 0; ay < H; ay++){
        for (ax = 0; ax < W; ax++){
            scanf("%d", &a[ay][ax]);
        }
    }
    for (by = 0; by < H; by++){
        for (bx = 0; bx < W; bx++){
            scanf("%d", &b[by][bx]);
        }
    }
    scanf("%d%d", &h, &w);
    for (cy = 0; cy < h; cy++){
        for (cx = 0; cx < w; cx++){
            scanf("%d", &c[cy][cx]);
        }
    }

    flag = 0;
    max = -100 * 50 * 50;
    for (by = 0; by < H - h + 1; by++){
        for (bx = 0; bx < W - w + 1; bx++){
            for (cy = 0; cy < h; cy++){
                for (cx = 0; cx < w; cx++){
                    if (b[by + cy][bx + cx] != c[cy][cx]){
                        goto UNMATCH;
                    }
                }
            }
            sum = 0;
            for (cy = 0; cy < h; cy++){
                for (cx = 0; cx < w; cx++){
                    sum += a[by + cy][bx + cx];
                }
            }
            if (max < sum){
                max = sum;
                flag = 1;
            }
UNMATCH:;
        }
    }
    
    if (flag == 0){
        printf("NA\n");
    }
    else {
        printf("%d\n", max);
    }
    return (0);
}


