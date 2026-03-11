#include <stdio.h>
int main(){
    int a=0,i,n,arr[200001],sum1=0,sum2=0;
    scanf("%d",&n);
    for(i=0;i<3*n;i++)
        scanf("%d",&arr[i]);
    if(n==1) arr[0]=0;
    else {
        a=1;
        while(a<n){
            if(a==n-1) arr[n*3-1]=0;
            arr[a]=0;
            a++;
        }
    }
    if(n==2) a=(3*n)/2;
    else a=(3*n)/2+1;
    for(i=0;i<3*n;i++){
        if(i<a) sum1+=arr[i];
        else sum2+=arr[i];
    }
    printf("%d\n",sum1-sum2);
    return 0;
}
