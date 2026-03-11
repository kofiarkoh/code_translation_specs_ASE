#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int main(void){
    int n,k,ans=0,a[11]={};
    scanf("%d%d",&n,&k);
    for (int i=0; i<11; i++){
        for (int j=1; j<6; j++){
            a[(((i*7)+j)*9+72)%11]++;
        }
    }
    ans+=(n-2)/11*a[k];
    
    //printf("%d\n",ans);
    
    for (long long i=(n-2)/11*11; i<n-2; i++){
        //printf("%lld\n",i);
        if ( (((i*7)+1)*9+72)%11==k || (((i*7)+2)*9+72)%11==k || (((i*7)+3)*9+72)%11==k || (((i*7)+4)*9+72)%11==k || (((i*7)+5)*9+72)%11==k){
            ans++;
        }
        //printf("%lld %lld %lld %lld %lld\n",(((i*7)+1)*9+72)%11,(((i*7)+2)*9+72)%11,(((i*7)+3)*9+72)%11,(((i*7)+4)*9+72)%11,(((i*7)+5)*9+72)%11);
    }
    printf("%d\n",ans);
	return 0 ;
}