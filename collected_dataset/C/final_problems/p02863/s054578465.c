#include<stdio.h>
#include<stdlib.h>

typedef struct node{
    int a;
    int b;
}node;

int compare_ascending(const void* a, const void* b){
    return ((node*)a)->a - ((node*)b)->a;
}

int main(){
    int n, t;
    scanf("%d%d", &n, &t);
    node ar[n+1];
    for(int i=1; i<=n; i++){
        scanf("%d%d", &(ar[i].a), &(ar[i].b));
    }
    int d[n+1][7001];
    for(int i=0; i<=7000; i++){
        d[0][i] = 0;
    }
    for(int i=1; i<=n; i++){
        for(int j=0; j<=7000; j++){
            if(j < ar[i].a){
                d[i][j] = d[i-1][j];
            }else if(j-ar[i].a >= t){
                if(d[i][j-1] > d[i-1][j]){
                    d[i][j] = d[i][j-1];
                }else{
                    d[i][j] = d[i-1][j];
                }
            }else{
                if(d[i-1][j] < d[i-1][j-(ar[i].a)] + ar[i].b){
                    d[i][j] = d[i-1][j-(ar[i].a)] + ar[i].b;
                }else{
                    d[i][j] = d[i-1][j];
                }
            }
        }
    }
    /*for(int i=80; i>=0; i--){
        printf("[%3d] ", i);
        for(int j=n; j>=0; j--){
            printf("%3d ", d[j][i]);
        }
        printf("\n");
    }*/
    printf("%d", d[n][7000]);

    return 0;
}
