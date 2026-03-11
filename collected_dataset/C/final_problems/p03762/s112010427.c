#include <stdio.h>
#define div 1000000007

int main(void){
    int n,m,i,j,k,l;
    long long int sum=0;
    scanf("%d %d",&n,&m);
    long long int xarr[n];
    long long int yarr[m];
    
    for(i=0;i<n;i++){
        scanf("%lld",&xarr[i]);
    }
    for(i=0;i<m;i++){
        scanf("%lld",&yarr[i]);
    }

 

    for(i=0;i<n-1;i++){
        for(j=0;j<m-1;j++){
            for(k=i+1;k<n;k++){
                for(l=j+1;l<m;l++){
                    sum+=(((xarr[k]%div-xarr[i]%div)%div)*((yarr[l]%div-yarr[j]%div)%div))%div;
                    /*if(((xarr[k]%div-xarr[i]%div)%div)*((yarr[l]%div-yarr[j]%div)%div)
                    /(yarr[l]%div-yarr[j]%div)!=(xarr[k]%div-xarr[i]%div)%div)
                    {
                        printf("error!\n");
                    }*/
                    sum%=div;
                }
                sum%=div;
            }
            sum%=div;
        }
        sum%=div;
    }
    sum%=div;
    printf("%llu",sum);
}