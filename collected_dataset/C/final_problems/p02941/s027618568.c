#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <limits.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */
#define min(a,b) (a < b ? a: b)
#define max(a,b) (a > b ? a: b)

#define MAXN (200000)
#define MOD (998244353)


// 優先度付きキュー Priority Queue
// 最大を保持
// 二分ヒープで実装
typedef struct 
{
    int64_t pri;
    int64_t val;
}que;
que pq[2*MAXN+1];
int pq_size=0;

void PQadd(int64_t pri, int64_t val)
{
    pq_size++;
    pq[pq_size].pri = pri;
    pq[pq_size].val = val;
    int i = pq_size;
    while( (i>1) && (pq[i].pri > pq[i/2].pri))
    {
        que temp;
        temp = pq[i];
        pq[i] = pq[i/2];
        pq[i/2] = temp;
        i /= 2;
    }
}

que PQget(void)
{
    que ret;
    ret = pq[1];
    pq[1] = pq[pq_size];
    pq_size--;

    int i = 1;
    while(i*2 <= pq_size) // 葉ならやめる
    {
        int j;
        if(i*2 == pq_size) //左の子しかいない
        {
            j = i*2;
        }else
        {
            j = pq[i*2].pri > pq[i*2+1].pri ? i*2 : i*2+1;
        }

        if(pq[i].pri < pq[j].pri)
        {
            que temp;
            temp = pq[i];
            pq[i] = pq[j];
            pq[j] = temp;
            i = j;
        }else
        {
            break;
        }
        
    }

    return ret;
}

int a[MAXN];
int b[MAXN];

int main(void)
{
    int n;
    scanf("%d",&n);

    for(int i=0;i<n;i++) scanf("%d",&(a[i]));
    for(int i=0;i<n;i++) scanf("%d",&(b[i]));

    for(int i=0;i<n;i++)
    {
        if(a[i] > b[i])
        {
            printf("-1\n");
            return 0;
        }else if(a[i] < b[i])
        {
            PQadd(b[i],i);
        }
    }

    int cnt = 0;
    while(pq_size)
    {
        int i = PQget().val;
        int j = (i+n-1)%n;
        int k = (i+1)%n;
        int diff = b[j] + b[k];

        if(b[i] < diff)
        {
            printf("-1\n");
            return 0;
        }

        if(b[i]%diff == a[i]%diff)
        {
            cnt += (b[i]-a[i])/diff;
            b[i] = a[i];
        }else{
            cnt += b[i]/diff;
            b[i] %= diff;
            if(b[i] < a[i])
            {
                printf("-1\n");
                return 0;
            }
            PQadd(b[i],i);
        }
    }

    printf("%d\n",cnt);
    return 0;
}





