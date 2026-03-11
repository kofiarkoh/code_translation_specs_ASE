#include<stdio.h>

int plass( int sum , int count_roop, int X) {
    count_roop++;
    /*printf("%d\n", count_roop);*/
    int x, count = 0;
    if ( (5 - count_roop) * 1000 >= sum && count_roop <= 3 ) {
        x = sum - (4 - count_roop) * 1000;
        if ( x < 0) x = 0;
        /*if ( X > x) x = X;*/
        /*printf("%d %d %d %d\n", x, count_roop, sum, count);*/
        while ( x <= sum && x <= 1000) {
            if ( x == sum ) count++;
            else count += plass(sum - x, count_roop, x);
            x++;
        }
    } else if ( count_roop == 4 && sum <= 1000) {
        /*printf("key\n");*/
        count++;
    }
    return count;
}

int main(void) {
    int n;
    while ( scanf("%d", &n) != EOF) {
        printf("%d\n", plass(n, 0, 0));
    }
    return 0;
}
