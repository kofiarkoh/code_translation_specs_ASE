#include <stdio.h>
#include <stdlib.h>
#define max(a, b) (a > b? a : b);

long long **row, **col, **point;

void swap(long long *a, long long *b)
{
    long long tmp = *a;
    *a = *b;
    *b = tmp;
}

void sort(long long **array, long long length)
{
    for (int i = 0; i < length - 1; i++)
    {
        for (int j = 0; j < length - i - 1; j++)
        {
            if (array[j][1] < array[j + 1][1])
            {
                swap(&array[j][0], &array[j + 1][0]);
                swap(&array[j][1], &array[j + 1][1]);
            }
        }
    }
}

int main(void)
{
    long long H, W, M;
    scanf("%lld %lld %lld", &H, &W, &M);
    long long maxNum = 0;

    row = (long long**)calloc(H, sizeof(long long*));
    for (int i = 0; i < H; i++)
        row[i] = (long long*)calloc(2, sizeof(long long));

    col = (long long**)calloc(W, sizeof(long long*));
    for (int i = 0; i < W; i++)
        col[i] = (long long*)calloc(2, sizeof(long long));

    point = (long long**)calloc(H, sizeof(long long));
    for (int i = 0; i < H; i++)
        point[i] = (long long*)calloc(W, sizeof(long long));

    for (long long i = 0; i < M; i++)
    {
        long long a, b;
        scanf("%lld %lld", &a, &b);

        row[a - 1][0] = a;
        row[a - 1][1]++;
        col[b - 1][0] = b;
        col[b - 1][1]++;

        point[a - 1][b - 1] = 1;
    }

    sort(row, H);
    sort(col, W);

    //printf("ok\n");
    long long tmpR = 0, tmpC = 0;
    for (int i = 0; i < H; i++)
    {
        if (tmpR > row[i][1])
            break;

        tmpR = row[i][1];

        for (int j = 0; j < W; j++)
        {
            if (tmpC > col[j][1])
                break;

            tmpC = col[j][1];

            long long total = tmpR + tmpC;

            if (point[row[i][0] - 1][col[j][0] - 1] == 1)
            {
                total--;
                maxNum = max(maxNum, total);
            }
            else
            {
                printf("%lld", total);
                return 0;
            }
        }
    }

    printf("%lld", maxNum);


    for (int i = 0; i < H; i++)
        free(row[i]);
    free(row);

    for (int i = 0; i < W; i++)
        free(col[i]);
    free(col);

    for (int i = 0; i < H; i++)
        free(point[i]);
    free(point);
    return 0;
}
