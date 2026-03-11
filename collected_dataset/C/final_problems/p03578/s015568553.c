#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

int sort(const void *a, const void *b){
    return *(int*)a-*(int*)b ;
}

int main(void){
    int n;
    scanf("%d",&n);
    int d[n];
    for (int i=0; i<n; i++){
        scanf("%d",&d[i]);
    }
    int m;
    scanf("%d",&m);
    int t[m];
    for (int i=0; i<n; i++){
        scanf("%d",&t[i]);
    }
    qsort (d,n,sizeof(int),sort); 
    qsort (t,m,sizeof(int),sort);
    int count=0;
    for (int i=0; i<n; i++){
        if (d[i]==t[count]){
            count++;
        }
    }
    if (count==m){
        printf("YES\n");
    } else {
        printf("NO\n");
    }
	return 0 ;
}