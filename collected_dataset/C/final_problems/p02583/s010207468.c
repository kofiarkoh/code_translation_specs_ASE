#include<stdio.h>

int main() {
    int i,j,k, n, L[100], count=0;
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%d",&L[i]);
    }
    for(i=0;i<n-2;i++){
        for(j=1+i;j<n-1;j++){
            for(k=1+j;k<n;k++){
                if(L[i]+L[j]>L[k] && L[i]+L[k]>L[j] && L[k]+L[j]>L[i] && L[i] != L[j] && L[i] != L[k] && L[k] != L[j]){
                    count++;
                }
            }
        }
    }
    printf("%d\n", count);
    return 0;
}