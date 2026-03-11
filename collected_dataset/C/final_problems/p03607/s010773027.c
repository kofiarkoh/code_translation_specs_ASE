#include <stdio.h>
#include <stdlib.h>
 
int asc(const void *a, const void *b){
    long *A = (long *)a;
    long *B = (long *)b;
    if(*A > *B)
        return 1;
    if(*A < *B)
        return -1;
    return 0;
}
 
int main(){
    int N, i, cnt=0, tag=0, ans=0;
    long int a[100000];
 
    scanf("%d", &N);
    for(i=0; i<N; i++){
        scanf("%ld", &a[i]);
    }
 
    qsort(a, N, sizeof(long int), asc);
 
    for(i=0; i<N; i++){
        if(a[i] == a[i+1]){
            cnt++;
        }
        else{
            if(cnt % 2 == 0 || cnt == 0)
                ans++;
            cnt = 0;
        }
    }
 
    printf("%d\n", ans);
 
    return 0;
}