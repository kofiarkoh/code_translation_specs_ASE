#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>

#define N_MAX 3000

typedef struct
{
    int x;
    int y;
}Point;

int N;
int K;

Point P[N_MAX];
Point T[N_MAX];

int compX(const void *a, const void *b)
{
    if(((Point *)a)->x == ((Point *)b)->x)
    {
        return ((Point *)a)->y - ((Point *)b)->y;
    }
    return ((Point *)a)->x - ((Point *)b)->x;
}

int compA(const void *a, const void *b)
{
    return ((Point *)b)->x * ((Point *)a)->y - ((Point *)a)->x * ((Point *)b)->y;
}

bool eq(const Point a, const Point b)
{
  return b.x * a.y == a.x * b.y;
}

int check(int p, int M)
{
    int maxl = 1;
    int l = 1;
    Point pre;
    int i;
    
    qsort(T, M, sizeof(Point), compA);
    pre = T[0];
    
    for(i = 1; i < M; i++)
    {
        if(eq(pre, T[i]))
        {
            l++;
            if(maxl > l)
            {
                maxl = maxl;
            }
            else
            {
                maxl = l;
            }
        }
        else
        {
            l = 1;
        }
        pre = T[i];
    }
    return maxl;
}


int solve()
{
    int k = 0;
    int v = 0;
    int i;
    int j;
    
    for (i = 0; i < N; i++)
    {
        k = 0;
        v = 0;
        for (j = i + 1; j < N; j++)
        {
            if ( P[i].x == P[j].x )
            {
                v++;
            }
            else
            {
                T[k] = P[j];
                T[k].x -= P[i].x;
                T[k].y -= P[i].y;
                k++;
            }
        }
        if ( 1 + v >= K ) return 1;
        if ( 1 + check(i, k) >= K ) return 1;
    }
    return 0;
}

int main()
{
    int i;

    scanf("%d%d", &N, &K);

    for(i = 0; i < N; i++)
    {
        scanf("%d%d", &P[i].x, &P[i].y);
    }
    qsort(P, N, sizeof(Point), compX);
    
    printf("%d\n", solve());
    
    return 0;
}
