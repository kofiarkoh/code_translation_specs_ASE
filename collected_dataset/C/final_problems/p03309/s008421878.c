#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>



int main(void){
    int n,ans,tmp;
    scanf("%d",&n);
    int a1[n],a[n];
    for (int i=0; i<n; i++){
        scanf("%d",&a1[i]);
        a[i]=a1[i]-(i+1);
    }
    
    for (int i=0; i<n; i++){
        tmp=0;
        for (int j=0; j<n; j++){
            tmp+=abs(a[j]-a[i]);
            if (tmp>ans && i!=0){
                break;
            }
        }
        if (i==0){
            ans=tmp;
        }else if (tmp<ans){
            ans=tmp;
        }
    }
    printf ("%d\n",ans);
    return 0 ;
}