#include <stdio.h>
#include <stdlib.h>
 
int main(void){
    int a[1000], n, x, min, ans = 0;
     for (int i = 0; i < 2; ++i) {
         scanf("%d", &a[i]);
    }
    n = a[0];
    x = a[1];
    for (int i = 0; i < n; ++i) {
         scanf("%d", &a[i]);
    }
    min = a[0];
    for(int i=0; i<n; i++){
        if(min>a[i]) min = a[i];
        x = x - a[i];
        ++ans;
    }
    while(x>=min){
        x = x-min;
        ++ans;
    }
    printf("%d", ans);
}