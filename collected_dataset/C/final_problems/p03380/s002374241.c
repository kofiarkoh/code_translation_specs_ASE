#include <stdio.h>
#include <stdlib.h>

int asc(const void *a, const void *b) {
  return *(int *)a - *(int *)b;
}



long com(int n, int r){
    long ans = 1;
    for (int i = n; i > n - r; --i) {
        ans = ans*i;
    }
    for (int i = 1 ; i < r + 1; ++i) {
        ans = ans / i;
    }
    return ans;
}

int main(){

    int n,a[100001];
    int i,j,t;
    int cn,cr;
    long max=0;
    long ans;

    scanf("%d", &n);
    for(i=0;i<n;i++){
        scanf("%d", &a[i]);
    }

    qsort(a, n, sizeof(int), asc);


    for(i=0;i<n-1;i++){
        for(j=i+1;j<n;j++){
            if(a[i]>a[j]-a[i] && a[j]-a[i]>=0){
                ans = com(a[j],a[j]-a[i]);
            }else{
                ans = com(a[j],a[i]);
            }
            //printf("%d %d %ld\n", a[j],a[i],ans);
            if(max<ans){
                max = ans;
                cn=a[j]; cr=a[i];
            }
        }
    }

    printf("%d %d\n", cn,cr);

    return 0;
}
