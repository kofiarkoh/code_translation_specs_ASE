#include<stdio.h>

int main(void){
    int N = 0, L = 0;
    int sweet = 0, pie = 0;
    int i = 1, j = 1;

    scanf("%d %d", &N, &L);

    sweet = L*L;

    for(i = 2; i <= N; i++){
        if(sweet > (L+i-1)*(L+i-1)){
            sweet = (L+i-1)*(L+i-1);
            j = i;
        }
    }

    for(i = 1; i <= N; i++){
        pie += L + i - 1;
    }

    pie -= L + j -1;

    printf("%d", pie);

    return 0;
}