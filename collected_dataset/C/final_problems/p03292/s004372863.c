#include <stdio.h>

int main(void)
{
    int a[3],i,j,t;
    for(i=0;i<3;i++)scanf("%d",&a[i]);
    for(i=0;i<2;i++){
        for(j=2;j>i;j--){
            if(a[j]<a[j-1]){
                t=a[j];
                a[j]=a[j-1];
                a[j-1]=t;
            }
        }
    }
    printf("%d\n",a[2]-a[1]+(a[1]-a[0]));
    return 0;
}