#include <stdio.h>

int main(void){
    int P = 200003;
    int n = 0;
    int a[200000];
    int sum = 0;
    int prdct = 0;

    scanf("%d",&n);

    for(int i = 0; i< n; i++){
        scanf("%d",&a[i]);
    }

    for(int i = 0; i < n; i++){
        for(int j = i+1; j < n; j++){
            if(*(a+i) && *(a+j)){
                sum += (*(a+i) * *(a+j)) % P;
            }
        }
    }

    printf("%d", sum);
    return 0;
}