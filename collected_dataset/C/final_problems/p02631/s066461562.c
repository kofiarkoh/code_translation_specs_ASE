#pragma GCC optimize("Ofast")
#include<stdio.h>
int main(void)
{
    int n = 0, a = 0, b;
    int d[200000];
    char c=getchar_unlocked();
    while(c<='9'&&c>='0') {n=(n<<1)+(n<<3)+c-'0'; c=getchar_unlocked();}
    int* e = d + n;
    do
    {
        d[n] = 0;
        c=getchar_unlocked();
        while(c<='9'&&c>='0') {d[n]=(d[n]<<1)+(d[n]<<3)+c-'0'; c=getchar_unlocked();}
        a ^= d[n];
    } while(--n);
    while(d != e)
    {
        printf("%d ", a ^ *e);
        --e;
    }
    return 0;
}
