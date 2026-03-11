#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
int acs(const void *a, const void *b){return *(int*)a - *(int*)b;} /* 1,2,3,4.. */
int des(const void *a, const void *b){return *(int*)b - *(int*)a;} /* 8,7,6,5.. */

#define MOD (998244353)

// BIT
typedef int BIT_T;
//BIT_T* bit;
BIT_T bit[200001];
int bitN;
void BITinit(int n)
{
    bitN = n;
//    bit = (BIT_T*)malloc( (bitN+1)*sizeof(BIT_T));
}

void BITadd(int i,BIT_T val)
{
    for(;i<=bitN;i+=i&-i) bit[i]+=val;
}

BIT_T BITget(int i)
{
    BIT_T val = 0;
    for(;i;i-=i&-i) val += bit[i];
    return val;
}

void BITreset(void)
{
    memset(bit, 0, sizeof(BIT_T)*(bitN+1));
}

typedef struct 
{
    int x;
    int y;
}P;

int acsY(const void *a, const void *b){return ((P*)a)->y - ((P*)b)->y;} /* 1,2,3,4.. */
int acsX(const void *a, const void *b){return ((P*)a)->x - ((P*)b)->x;} /* 1,2,3,4.. */

// 2^n-1 % MOD
uint64_t getC(int n)
{
    uint64_t c;
    c = 1;
    for(int i=0;i<n;i++) c = (c*2) % MOD;
    return (c + MOD -1) % MOD;
}

P ps[200000];
int a[200000]={0};
int b[200000]={0};
int c[200000]={0};
int d[200000]={0};

int main(void)
{
    int n;
    uint64_t sum;
    scanf("%d",&n);
    
    for(int i=0;i<n;i++)scanf("%d %d", &(ps[i].x), &(ps[i].y));
    qsort(ps, n, sizeof(P), acsY);
    int j=1;
    for(int i=0;i<n;i++) ps[i].y = j++;
    qsort(ps, n, sizeof(P), acsX);
        
    //BA
    //CD
    BITinit(n);
    for(int i=0;i<n;i++)
    {
        int g = BITget(ps[i].y);
        b[i] += g;
        c[i] += i-g;
        BITadd(ps[i].y,1);
    }
    BITreset();
    for(int i=n-1;i>=0;i--)
    {
        int g = BITget(ps[i].y);
        a[i] += g;
        d[i] += (n-1-i)-g;
        BITadd(ps[i].y,1);
    }

    uint64_t t;
    t = 0;
    for(int i=0;i<n;i++)
    {
        // ABCD
        // ABC
        // AB D
        // A CD
        //  BCD
        //  B D
        // A C
        uint64_t ca = getC(a[i]);
        uint64_t cb = getC(b[i]);
        uint64_t cc = getC(c[i]);
        uint64_t cd = getC(d[i]);
        t += (((((ca * cb)%MOD)*cc)%MOD)*cd)%MOD;
        t %= MOD;
        t += ((((ca * cb)%MOD)*cc)%MOD);
        t %= MOD;
        t += ((((ca * cb)%MOD)*cd)%MOD);
        t %= MOD;
        t += ((((ca * cc)%MOD)*cd)%MOD);
        t %= MOD;
        t += ((((cb * cc)%MOD)*cd)%MOD);
        t %= MOD;
        t += (cb * cd)%MOD;
        t %= MOD;
        t += (ca * cc)%MOD;
        t %= MOD;
        //自分を含む組み合わせ
        t += getC(n-1)+1;
        t %= MOD;
    }
    printf("%ld\n", t);
    
    return 0;
}

