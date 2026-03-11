#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int main(void){
    long long d,g;
    scanf("%lld%lld",&d,&g);
    int p[d],c[d];
    for (int i=0; i<d; i++){
        scanf("%d%d",&p[i],&c[i]);
    }
    long long ans=LLONG_MAX;
    for (int j=0; j<d; j++){
        long long q=0,score=0;
        for (int i=j; i<d; i++){
            int count=0;
            while(score<g && count<p[d-i-1]){
                score+=100*(d-i);
                q++;
                count++;
            }
            if (count==p[d-i-1]){
                score+=c[d-i-1];
           }
        }
        for (int i=0; i<j; i++){
            int count=0;
            while(score<g && count<p[d-i-1]){
                score+=100*(d-i);
                q++;
                count++;
            }
            if (count==p[d-i-1]){
                score+=c[d-i-1];
           }
        }
        if (ans>q){
            ans=q;
        }
    }
    printf("%lld\n",ans);
	return 0 ;
}