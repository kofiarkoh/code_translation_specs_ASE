#include<stdio.h>
int main(void){
    int n,a[100000],i,max,min,j,sum=0;
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%d",&a[i]);
    }
    max=min=a[0];
    for(j=0;j<n;j++){
        if(max<a[j])max=a[j];
        if(min>a[j])min=a[j];
        sum+=a[j];
    }
    printf("%d %d %d\n",min,max,sum);

    return 0;
}