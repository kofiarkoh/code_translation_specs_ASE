#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#define MOD 1000000009
#define MAX(a,b) (a>b?a:b)
#define MAX3(a,b,c) MAX(a,MAX(b,c))
#define MIN(a,b) (a<b?a:b)
#define MIN3(a,b,c) MIN(a, MIN(b,c))
typedef long long lli;
typedef unsigned short int usi;

// 昇順
int cmp(const void *a, const void *b) {
    return *(int*)a - *(int*)b;
}

int main()
{
    int a, b;
    int count[5]={0};
    int i;

    for(i=0; i<3; i++) {
        scanf("%d %d",&a, &b);
        count[a]++;
        count[b]++;
    }
    for(i=1; i<=4; i++) {
        if(count[i]>2 || count[i]==0) {
            printf("NO");
            return 0;
        }
    }
    printf("YES");
    return 0;
}
