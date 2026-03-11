#include <stdio.h>
#include <stdlib.h>
#define max(a, b) (a > b? a : b);

long long row[300000][2], col[300000][2], *point;

void swap(long long *a, long long *b)
{
    long long tmp = *a;
    *a = *b;
    *b = tmp;
}

void sort(long long array[][2], long long length)
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
    point = (long long*)calloc(H * W, sizeof(long long));
    for (long long i = 0; i < M; i++)
    {
        long long a, b;
        scanf("%lld %lld", &a, &b);

        row[a - 1][0] = a;
        row[a - 1][1]++;
        col[b - 1][0] = b;
        col[b - 1][1]++;

        point[a + b - 2] = 1;
    }

    sort(row, H);
    sort(col, W);

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

            if (point[row[i][0] + col[j][0] - 2] == 1)
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

    free(point);
    return 0;
}