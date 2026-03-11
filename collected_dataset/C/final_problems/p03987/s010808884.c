#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int main(void){
    int n;
    scanf("%d",&n);
    long long a[n+2],l[n+2],r[n+2],ans=0;
    for (int i=0; i<n; i++){
        scanf("%lld",&a[i]);
    }
    for (int i=0; i<n; i++){
        long long tmp=i-1;
        while(tmp!=-1 && a[tmp]>a[i]){
            tmp=l[tmp];
        }
        l[i]=tmp;
    }
    for (int i=n-1; i>=0; i--){
        long long tmp=i+1;
        while(tmp!=n && a[tmp]>a[i]){
            tmp=r[tmp];
        }
        r[i]=tmp;
    }
    for (int i=0; i<n; i++){
        ans+=a[i]*(i-l[i])*(r[i]-i);
    }
    printf("%lld\n",ans);
	return 0 ;
}