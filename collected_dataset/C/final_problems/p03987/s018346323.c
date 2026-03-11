#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int sort(const void *a, const void *b){
    if( *(long long int*)a - *(long long int*)b < 0 )
        return -1;
    if( *(long long int*)a - *(long long int*)b > 0 )
        return 1;
    return 0;
}

int main(void){
    int n,count=2;
    scanf("%d",&n);
    long long a[n+2],b[n+2],c[n+2],ans=0,l,r;
    a[0]=0;
    a[n+1]=0;
    b[n+1]=n+1;
    b[0]=-1;
    c[0]=0;
    c[1]=n+1;
    for (int i=1; i<=n; i++){
        scanf("%lld",&a[i]);
        b[a[i]]=i;
    }
    for (long long i=1; i<=n; i++){
        l=0,r=0;
        long long max=count-1;
        long long min=0;
        while(1){
            long long mid=min+(max-min)/2;
            if (c[mid]<b[i]){
                min=mid+1;
            } else {
                max=mid-1;
            }
            if (c[mid]>b[i] && c[mid-1]<b[i]){
                l=b[i]-c[mid-1];
                r=c[mid]-b[i];
                break;
            }
        }
        ans+=i*l*r;
        c[count]=b[i];
        count++;
        qsort (c,count,sizeof(long long),sort);
    }
    printf("%lld\n",ans);
	return 0 ;
}