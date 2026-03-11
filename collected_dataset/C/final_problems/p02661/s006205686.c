#include <stdio.h>
#define N 200001
#include <stdlib.h>

int compare(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}


int main()//リニューアル
{

    int a;

    long long Big[N];
    long long small[N];

    scanf("%d", &a);

    for (int i = 0; i < a; i++)
    { //道を読み取る

        scanf("%lld %lld", &small[i],&Big[i]);

    }
    qsort(Big, a, sizeof(long long), compare);
    qsort(small,a, sizeof(long long), compare);

    if(a%2==0){
        long long distance =(Big[a / 2] + Big[a / 2-1] )- (small[a / 2]+small[a / 2-1]);//わる２して普通に差
        printf("%lld", distance+1);
    }
    else{//基
        printf("%lld", Big[a / 2] - small[a / 2] + 1);
    }


    printf("\n");
}
