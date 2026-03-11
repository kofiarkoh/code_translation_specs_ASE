#include<stdio.h>
#include<math.h>
#include<stdlib.h>

#define N 200000

int n, a[N], x[N];

int main(){
    int allXor=0; 
    scanf("%d",&n);
    for (int i = 0; i < n; i++){
        scanf("%d", &a[i]);
        allXor = allXor^a[i];
    }

    for (int i = 0; i < n; i++){
        x[i] = allXor^a[i];
        printf("%d ",x[i]);
    }
    printf("\n");

    return 0;
}
