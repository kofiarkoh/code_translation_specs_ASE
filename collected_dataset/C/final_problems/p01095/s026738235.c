#include <stdio.h>
#define MAX 7368791
char flag[MAX+1];
int m;
int n;
int main(void){
    int i,j;
    while(1){
        scanf("%d%d", &m, &n);
        if(m==0 && n==0) break;
        for(i=0; i<=MAX; i++) flag[i] = 0;
        for(i=0; i<n; i++){
            for(j=m; j<=MAX; j += m)
                flag[j] = 1;
            while(flag[m]==1) m++;
        }
        printf("%d\n",m);
    }
    return 0;
}

