#include <stdio.h>

int main(){
    int N, A;
    int sisa;

    scanf("%d",&N);
    scanf("%d",&A);

    sisa = N % 500;

    if(A >= sisa){
        printf("Yes\n");
    }
    else{
        printf("No\n");
    }

return 0;
}
