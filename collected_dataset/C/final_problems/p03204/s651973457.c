#include <stdio.h>
#include <stdlib.h>

#define MAX_N (200000)
#define imin(a,b) ((a)<(b)?(a):(b))

int cache[MAX_N + 1];

int qsort_compare_int_x2(const void *a, const void *b)
{
    int *a_int, *b_int;
    a_int = (int *) a;
    b_int = (int *) b;
    if (a_int[0] > b_int[0]) {return  1;}
    if (a_int[0] < b_int[0]) {return -1;}
    if (a_int[1] > b_int[1]) {return  1;}
    if (a_int[1] < b_int[1]) {return -1;}
    return 0;
}

int move(int room, int n, int a[MAX_N * 2][2], int a_marker[MAX_N + 1])
{
    int i, j;
    int count;
    int now;
    int next;
    int rooms = (n / 2) + 1;
    int min;

    int c_tmp[MAX_N + 1][2];

    char b[MAX_N + 1]; // 1 = 訪れた, 2 = 行ける(隣接)

    if (cache[room] != 0) {return cache[room];}

    for (i = 1; i <= rooms; i++) {
        b[i] = 0;
    }
    for (i = 2; i <= rooms; i++) {
        c_tmp[i][0] = 0;
        c_tmp[i][1] = 0;
    }

    now = room;

    count = 0;
    while(1) {
        b[now] = 1;
        min = imin(min, now);
        for (i = a_marker[now]; a[i][0] == now; i++) {
            // 今いる部屋との接続
            next = a[i][1];
            if (b[next] != 1) {b[next] = 2;}
        }
        for (i = 1; i <= rooms; i++) {
            if (b[i] == 2) {
                if (i == 1) {
                    for (j = 2; j <= rooms; j++) {
                        if (c_tmp[j][0] != 0) {
                            cache[j] = count - c_tmp[j][0]; 
                        }
                    }
                    return count + 1;
                }
                now = i;
                if (now < min) {
                    c_tmp[now][0] = count;
                    c_tmp[now][1] = min;
                }
                count++;
                break;
            }
        }
    }
}

int main(int argc, char *argv[])
{
    int n, i, j;
    int a[MAX_N * 2][2];
    int a_marker[MAX_N + 1];
    int val[MAX_N + 1];

    scanf("%d", &n);
    for (i = 0; i < 2 * (n - 1); i += 2) {
        scanf("%d %d", &a[i][0], &a[i][1]);
        a[i + 1][1] = a[i][0];
        a[i + 1][0] = a[i][1];
    }

    for (i = 1; i <= n; i++) {
        cache[i] = 0;
    }

    qsort(a, 2 * (n - 1), sizeof(int) * 2, qsort_compare_int_x2);

    i = 0;
    for (j = 1; j <= n; j++) {
        if (a[i][0] == j) {a_marker[j] = i;}
        while(a[i][0] == j) {i++;}
    }
/*
    for (i = 2; i <= n; i++) {
       if (i != 2) {printf(" ");}
       printf("%d", move(i, (n - 1) * 2, a, a_marker));
    }*/
    for (i = n; i > 1; i--) {
        val[i] = move(i, (n - 1) * 2, a, a_marker);
    }
    for (i = 2; i <= n; i++) {
       if (i != 2) {printf(" ");}
       printf("%d", val[i]);
    }

    printf("\n");

    return 0;
}