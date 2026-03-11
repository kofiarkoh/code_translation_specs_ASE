#include<stdio.h>

void sort(int *a, int n){
    int i;

    for(i = 0; i < n; i++){
        int j;
        for(j = n - 2; j >=  i; j--){
            if(a[j] > a[j + 1]){
                int tmp;
                tmp = a[j];
                a[j] = a[j + 1];
                a[j + 1] = tmp;
            }
        }
    }
}

void ary_cpy(int *a, int *b, int n){
    int i;

    for(i = 0; i < n; i++){
        b[i] = a[i];
    }
}

int max(int *a, int n, int j){
    int b[200000];

    ary_cpy(a, b, n);

    b[j] = 0;

    sort(b, n);

    printf("%d\n", b[n - 1]);

    j++;

    if(j < n){
        return max(a, n, j);
    }

    return 0;
}

int main(){
    int n;
    int a[200000];
    int b[200000];

    scanf("%d", &n);

    int i;
    for(i = 0; i < n; i++){
        scanf("%d", &a[i]);
    }

    int j = 0;
    max(a, n, j);

    return 0;
}