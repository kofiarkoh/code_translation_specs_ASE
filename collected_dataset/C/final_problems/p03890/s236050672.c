#include <stdio.h>
#include <math.h>
int main(void){
    int n,m,i,j;
    int a[10],tmp;
    scanf("%d",&n);
    m=n;
    n=pow(2,n);
    for(i=0;i<n;i++){
        scanf("%d",&a[i]);
    }
    for(i=0;i<m;i++){
        tmp=pow(2,i);
        for(j=0;j<n;j++){
            if(a[j]>a[j+tmp]){
                a[j]=a[j]-a[j+tmp];
            }else if(a[j]<a[j+tmp]){
                a[j]=a[j+tmp]-a[j];
            }
            a[j+tmp]=0;
            j++;
        }
        n=n/2;
    }
    printf("%d\n",a[0]);
    return 0;
}
