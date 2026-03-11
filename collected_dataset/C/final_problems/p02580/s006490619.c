#include <stdio.h>
#include <stdlib.h>
#define max(a, b) (a > b? a : b);

long long row[300000], col[300000], point[300000];

int binary_search(long long target, long long array[], long long n)
{
    long long fn = 0, bn = n - 1, tmp;
    while (fn <= bn)
    {
        tmp = (fn + bn) / 2;
        if (array[tmp] < target) {fn = tmp + 1;} else if (array[tmp] > target) {bn = tmp - 1;} else {return tmp;}
    }
    return fn;
}

int compare(const void *a, const void *b) {return *(long long*)a > *(long long*)b ? 1 : (*(long long*)a == *(long long*)b ? 0 : -1);}

int main(void)
{
    long long H, W, M;
    scanf("%lld %lld %lld", &H, &W, &M);
    long long maxRow = 0, maxCol = 0;

    for (long long i = 0; i < M; i++)
    {
        long long a, b;
        scanf("%lld %lld", &a, &b);
        row[a - 1]++;
        col[b - 1]++;

        if (row[a - 1] > maxRow)
            maxRow = row[a - 1];
        if (col[b - 1] > maxCol)
            maxCol = col[b - 1];

        point[i] = a * W + b;
    }

    long long r = maxRow + maxCol - 1;

    long long numRow = 0, numCol = 0;
    long long indexRow[M], indexCol[M];
    for (long long i = 0; i < H; i++)
    {
        if (row[i] == maxRow)
        {
            indexRow[numRow++] = i;
        }
    }

    for (long long i = 0; i < W; i++)
    {
        if (col[i] == maxCol)
        {
            indexCol[numCol++] = i;
        }
    }

    if (numRow*numCol>M) {printf("%lld", r+1); return 0;}
    qsort(point, M, sizeof(long long), compare);

    for (long long i = 0; i < numRow; i++)
    {
        for (long long j = 0; j < numCol; j++)
        {
            long long key = (indexRow[i]+1)*W+indexCol[j]+1;
            if (point[binary_search(key, point, M)] != key)
            {
                printf("%lld", r + 1);
                return 0;
            }
        }
    }

    printf("%lld", r);
    return 0;
}
