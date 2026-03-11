#include <stdio.h>
#define N 100000
int main(void){
    int n,i,a[N];
    scanf("%d",&n);
    for(i=0;i<n;i++) scanf("%d",&a[i]);
    for(i=0;i<n-1;i++){
        if(a[i]==a[i+1]) a[i]=0;
    }
    for(i=0;i<n;i++){
        if(a[i]!=0) {
            printf("%d",a[i]);
            if(i!=n-1) printf(" ");
             else printf("\n");
        }
    }
    return 0;
}
