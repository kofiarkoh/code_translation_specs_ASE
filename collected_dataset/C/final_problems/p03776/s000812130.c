#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

long long ncr(int a,int b){
    long long combinationans=1;
    for (int i=0; i<b; i++){
        combinationans=combinationans*(a-i)/(i+1);   
    }
    return (combinationans);
}

int sort(const void *a, const void *b){
    if( *(long long int*)a - *(long long int*)b < 0 )
        return 1;
    if( *(long long int*)a - *(long long int*)b > 0 )
        return -1;
    return 0;
}

int main(void){
    int n,a,b;
    scanf("%d%d%d",&n,&a,&b);
    long long v[n];
    for (int i=0; i<n; i++){
        scanf("%lld",&v[i]);
    }
    qsort (v,n,sizeof(long long),sort);
    
    long long count[n+2];
    for (int i=a; i<=b; i++){
        int countn=0,countr=0;
        for (int j=0; j<n; j++){
            if (v[i-1]==v[j]){
                countn++;
                if (j<i){
                    countr++;
                }
            }
        }
        count[i]=ncr(countn,countr);
    }
    
    double ans=0;
    long long ans2=0;
    
    long long anssum=0;
    int ansi=1;
    
    for (int i=a; i<=b; i++){
        long long sum=0;
        for (int k=0; k<i; k++){
            sum+=v[k];
        }
        if (anssum*i<sum*ansi){
            ans=sum/(double)i;
            anssum=sum;
            ansi=i;
            ans2=count[i];
        } else if (anssum*i==sum*ans){
            ans2+=count[i];
        }
    }
    printf("%lf\n%lld\n",ans,ans2);
	return 0 ;
}