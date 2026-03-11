#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */

#define MAXN (100000*2+1)
#define MOD (1000000007)

//Union Find
int uf[MAXN];
int ufSize[MAXN];
void UFinit(int n)
{
    for(int i=0;i<n;i++)
    {
        uf[i] = i;
        ufSize[i] = 1;
    }
}

int UFroot(int i)
{
    if(uf[i]==i) return i;
    uf[i] = UFroot(uf[i]);
    return uf[i];
}

// 元々根が同じならfalse 違うなら統合してtrue
bool UFunite(int i,int j)
{
    int ri = UFroot(i);
    int rj = UFroot(j);

    if(ri == rj) return false;
    ufSize[ri] += ufSize[rj];
    uf[rj] = ri;
    return true;
}

typedef struct
{
    int r;
    int c;
    int a;
}card;

card cs[MAXN];
int des_cs(const void *a, const void *b){return (*(card*)b).a - (*(card*)a).a;} /* 8,7,6,5.. */

bool close[MAXN];

int main(void)
{
    int n,h,w;
    scanf("%d %d %d",&n, &h, &w);

    for(int i=0;i<n;i++) scanf("%d %d %d",&(cs[i].r),&(cs[i].c),&(cs[i].a));
    qsort(cs, n, sizeof(card), des_cs);

    UFinit(h+w+1); // c は h+1からスタート

    uint64_t ans = 0;
    for(int i=0;i<n;i++) 
    {
        int rr = UFroot(cs[i].r);
        int cr = UFroot(cs[i].c+h);
        if(rr == cr)
        {
            //既に同じ連結成分
            if(!close[rr])
            {
                close[rr] = true;
                ans += cs[i].a;
            }
        }else
        {
            if( !(close[rr] && close[cr]) )
            {
                UFunite(rr, cr);
                close[rr] = close[rr] || close[cr];
                ans += cs[i].a;
            }
        }
    }
    printf("%ld\n",ans);

    return 0;
}



