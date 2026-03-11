#include <stdio.h>

int main(void)
{
    long int n=0,k=0,a=0,b=0,i=0,j=0,cnt=0;
    static int box[1000000]={};
    
    scanf("%ld %ld",&n,&k);
    
    for(i=0;i<n;i++){
        scanf("%ld %ld",&a,&b);
        for(j=0;j<b;j++){
            box[cnt]=a;
            cnt++;
        }
        if(cnt>k){
            printf("%d\n",box[k-1]);
            break;
        }
    }
    
    return 0;
}