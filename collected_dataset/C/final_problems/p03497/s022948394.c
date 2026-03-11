#include <stdio.h>
#include <stdlib.h>
static int compare(const void* a, const void* b)
{
    int aNum = *(int*)a;
    int bNum = *(int*)b;

    return aNum-bNum;
}
int main(void)
{
    int n, i, ans=0, a[200001], cnt[200001]={0}, kind=0, k;
    scanf("%d%d", &n, &k);
    for(i=0;i<n;i++){
        scanf("%d", &a[i]);
    }

    for(i=0;i<n;i++){
        if(cnt[a[i]]==0) kind++;
        cnt[a[i]]++;
    }
    qsort(cnt, 200001, sizeof(int), compare);
    i=0;
    while(kind>k){
        if(cnt[i]>0){
        ans=ans+cnt[i];
        kind--;
        }
        i++;
    }

    printf("%d\n", ans);
    return 0;
}
