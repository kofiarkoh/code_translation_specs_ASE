#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main(void){
    int n;
    long long int k;
    scanf("%d %lld",&n,&k);
    int ans=1;
    int a[n];
    int log[n];
    for(int i=0; i<n; i++){
        scanf("%d",&a[i]);
        log[i]=0;
    }
    log[0]=1;
    long long int count=0;
    int loop=0;
    while(1){
        ans=a[ans-1];
        count++;
        log[count]=ans;
        for(int i=0; i<count; i++){
            if(log[count]==log[i]){
                loop=count-i;
                break;
            }
        }
        if(loop!=0){
            break;
        }
    }
    k=k%loop;
    ans=1;
    while(k>0){
        ans=a[ans-1];
        k--;
    }
    printf("%d\n",ans);
    return 0;
}