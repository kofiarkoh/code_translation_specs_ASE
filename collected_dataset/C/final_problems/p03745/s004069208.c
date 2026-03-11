#include <stdio.h>
int main(void){
    int n, a[100000+5];
    scanf("%d", &n);
    for(int i=0; i<n; i++)scanf("%d", &a[i]);
    
    int ans=0;
    for(int i=0; i<n; i++){
        ans++;
        while(a[i]==a[i+1])i++;
        if(a[i]<a[i+1])while(a[i]<a[i+1])i++;
        else while(a[i]>a[i+1])i++;
        
    }
    printf("%d\n", ans);
    return 0;
}
