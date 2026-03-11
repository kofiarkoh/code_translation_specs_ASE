#include <stdio.h>


int main(void){
    int N;
    scanf("%d",&N);
    int a[N],b[N];
    long long int ans=0;
    for(int i=0;i<N;i++){scanf("%d %d",&a[i],&b[i]);if(a[i]<b[i])ans+=(long long int)b[i];}
    printf("%lld",ans);
    return 0;
}
