#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int gcd(int a,int b){int r,tmp;if(a < b){tmp = a;a = b;b = tmp;}r = a % b;while(r != 0){a = b;b = r;r = a % b;}return b;}
int lcm(int a,int b){int r;r = gcd(a,b);r = a / r * b;return r;}
int min(int a,int b){return a < b ? a : b;}
int main()
{
    int n,m,l,i,j;
    char s[100001],t[100001];
    char* ans;
    ans = (char*)malloc(sizeof(char) * 10000000001);
    memset(ans,0,sizeof(ans));
    scanf("%d %d %s %s",&n,&m,s,t);
    l = lcm(n,m);
    for (i = 0; i < n; i++)
    {
        ans[i * l / n] = s[i];
    }
    for (i = 0; i < m; i++)
    {
        if(ans[i * l / m] != 0 && ans[i * l / m] != t[i])break;
    }
    
    printf("%d",i == m ? l : -1);
    free(ans);
}