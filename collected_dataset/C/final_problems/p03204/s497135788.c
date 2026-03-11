#include <stdio.h>
#include <stdlib.h>

#define MAX_N (200000)

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

int move(int room, int n, int a[MAX_N * 2][2])
{
    int i;
    int count;
    int now;
    int next;
    int rooms = (n / 2) + 1;

    char b[MAX_N + 1]; // 1 = 訪れた, 2 = 行ける(隣接)

    for (i = 1; i <= rooms; i++) {
        b[i] = 0;
    }

    now = room;

    count = 0;
    while(1) {
        b[now] = 1;
        for (i = 0; i < n; i++) {
            if (a[i][0] < now) {continue;}
            if (a[i][0] > now) {break;}
            // 今いる部屋との接続
            next = a[i][1];
            if (b[next] != 1) {b[next] = 2;}
        }
        for (i = 1; i <= rooms; i++) {
            if (b[i] == 2) {
                if (i == 1) {return count + 1;}
                now = i;
                count++;
                break;
            }
        }
    }
}

int main(int argc, char *argv[])
{
    int n, i;
    int a[MAX_N * 2][2];

    scanf("%d", &n);
    for (i = 0; i < 2 * (n - 1); i += 2) {
        scanf("%d %d", &a[i][0], &a[i][1]);
        a[i + 1][1] = a[i][0];
        a[i + 1][0] = a[i][1];
    }

    qsort(a, 2 * (n - 1), sizeof(int) * 2, qsort_compare_int_x2);

    for (i = 2; i <= n; i++) {
       if (i != 2) {printf(" ");}
       printf("%d", move(i, (n - 1) * 2, a));
    }

    printf("\n");

    return 0;
}