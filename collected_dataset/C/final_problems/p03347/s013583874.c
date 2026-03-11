#include <stdio.h>
int main(void){
    int n,ans=0;
    scanf("%d",&n);
    int a[n+1];
    a[n]=0;
    for (int i=0; i<n; i++){
        scanf("%d",&a[i]);
    }
    for (int i=1; i<n; i++){
        if (a[i]>a[i-1]+1 || a[i]>i){
            printf("-1\n");
            return 0;
        } else if (a[i]>=a[i+1]){
            ans+=a[i];
        }
    }
    printf("%d\n",ans); 
    return 0;   
}