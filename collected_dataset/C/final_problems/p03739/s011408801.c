#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

int main(void){
    int n,ans=0,tmp=0;
    scanf("%d",&n);
    int a[n],sum[n];
    for (int i=0; i<n; i++){
        scanf("%d",&a[i]);
    }
    for (int i=0; i<n; i++){
        sum[i]=sum[i-1]+a[i];
        sum[0]=a[0];
        if (i%2==0){
            if (sum[i]<=0){
                tmp+=-1*sum[i]+1;
                sum[i]=1;
            }
        } else {
            if (sum[i]>=0){
                tmp+=sum[i]+1;
                sum[i]=-1;
            }
        }
    }
    for (long long i=0; i<n; i++){
        sum[i]=sum[i-1]+a[i];
        sum[0]=a[0];
        if (i%2==0){
            if (sum[i]>=0){
                ans+=sum[i]+1;
                sum[i]=-1;
            }
        } else {
            if (sum[i]<=0){
                ans+=-1*sum[i]+1;
                sum[i]=1;
            }
        }
    }    
    if (tmp<ans){
        ans=tmp;
    }
    printf ("%d\n",ans);
	return 0 ;
}