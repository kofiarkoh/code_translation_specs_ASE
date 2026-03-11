#include <stdio.h>
#include <stdlib.h>

int qsort_compare_obj(const void *a, const void *b)
{
    if (*(long long *) a - *(long long *)b > 0) {return 1;}
    if (*(long long *) a - *(long long *)b == 0) {return 0;}
    if (*(long long *) a - *(long long *)b < 0) {return -1;}
}

int main(int argc, char *argv[])
{
    int h, w, n;
    long long obj[200000];
    int x, y;
    int pos_x, pos_y;
    int i;
    int min_count = 0x7FFFFFFF;
    int count;
    int diff;

    scanf("%d %d %d", &h, &w, &n);
    for(i = 0; i < n; i++) {
        scanf("%d %d", &x, &y);
        obj[i] = (long long) x * 0x100000000 + y;
    }

    qsort(obj, n, sizeof(long long), qsort_compare_obj);

    pos_x = 1;
    pos_y = 1;

    min_count = w;
    //aoki-target-search
    for(i = 0; i < n; i++) {
        x = obj[i] / 0x100000000;
        y = obj[i] % 0x100000000;
        diff = (y - pos_y) - (x - pos_x);
        if (diff == 0) {pos_x++; continue;}
        if (diff < 0) { //ぶつかれる
            count = x - 1;
            //printf("Hit! in (%d, %d), (%d, %d), diff = %d, count = %d\n", pos_x, pos_y, x, y, diff, count);
            min_count = (min_count < count)?min_count:count;
        }
    }

    printf("%d\n", min_count);

    return 0;
}