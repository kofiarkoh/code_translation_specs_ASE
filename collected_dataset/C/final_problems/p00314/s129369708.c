#include<stdio.h>

int main(void){
        int n;
        scanf("%d", &n);

        int i, a[100];
        for(i=0; i<100; i++) a[i]=0;
        for(i=0; i<n; i++){
                int tmp;
                scanf("%d", &tmp);
                a[tmp-1]++;
        }

        for(i=99; i>0; i--) a[i-1] += a[i];

        int ans = 0;
        for(i=0; i<100; i++){
                if(a[i]>i) ans = i+1;
        }

        printf("%d\n", ans);
        return 0;
}
